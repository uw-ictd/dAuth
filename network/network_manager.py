from threading import Lock
from time import time, sleep
from collections import deque
from concurrent.futures import ThreadPoolExecutor
import threading
import bisect

# Possible improvements/directions:
#  - Add a 'smart' way of keeping QUs in the active queue, maybe specify frequent priorities?
#  - Add bandwidth limiter (lots of problems with this)
#  - flip comparator (or add init comparator?) for QueueUnit

# Small class that holds on to a priority, function, and arguments
class NetworkMessage:
    # Takes in the priorty, sending function, and args to the sending function
    def __init__(self, priority, send_func, *args, **kwargs):
        self.priority = priority
        self.send_func = send_func
        self.send_data = (args, kwargs)

    def __str__(self):
        return f"priorty: <{self.priority}>, send_func: <{self.send_func}>, send_data: <{self.send_data}>"

    # Runs the sending function with the given data
    def send(self):
        self.send_func(*self.send_data[0], **self.send_data[1])


# Wrapper class for a single priority queue unit
# Implemenets a comparable function for sorting
class QueueUnit:
    def __init__(self, priority):
        self.priority = priority
        self._queue = deque()

    def put(self, element):
        self._queue.append(element)

    def get(self):
        return self._queue.popleft()

    def empty(self):
        return not self._queue

    def size(self):
        return len(self._queue)
    
    def __lt__(self, other):
        return self.priority < other.priority

    def __str__(self):
        return "QueueUnit: " + str(self.priority)


# Priority queue intended to store NetworkMessages, higher values have higher priority
# Any interger can be used as a priority, implemeneted by mapping priorities to queues
# Thread safe, but using only one 'getter' will likely lead to better performance
class NetworkPriotyQueue:
    def __init__(self):
        # --- Internal ---
        # mapping of all priority queues
        self._priority_queues = {}

        # ordered list of priority queue keys
        self._priority_queue_keys = []

        # holds all the queues with messages
        self._active_queues = []

        # remaining messages
        self._qsize = 0

        # lock for critical areas
        self._lock = Lock()

    # Appends a new message to the appropriate queue
    def put(self, message):
        self._lock.acquire()

        # create a queue if one doesn't exists
        if not message.priority in self._priority_queues:
            self._priority_queues[message.priority] = QueueUnit(message.priority)
        
        # get the queue unit
        qu = self._priority_queues[message.priority]

        # if the queue unit doesn't have any messages, put it in the active queue
        if qu.empty():
            self._insert(self._active_queues, qu)

        # put the message in the queue and increment total
        qu.put(message)
        self._qsize += 1

        self._lock.release()
    
    # Finds and returns the next highest priority message, or None if not found
    def get(self):
        return self._next()
    
    def empty(self):
        return self._qsize < 1
    
    def size(self):
        return self._qsize
    
    def _next(self):
        # check that at least one message exists
        if self.empty():
            return None

        self._lock.acquire()

        # sanity check
        if len(self._active_queues) < 1:
            self._lock.release()
            return None
        
        # get the highest priority queue and pop a message from it
        qu = self._active_queues.pop()
        message = qu.get()
        self._qsize -= 1

        # add list back if it still has elements
        if not qu.empty():
            self._active_queues.append(qu)

        self._lock.release()

        return message

    # Do an insert into a sorted list
    def _insert(self, lst, e):
        bisect.insort(lst, e)

    def __str__(self):
        return "Network Prioirty Queue ({} total messages)\n  ".format(self.size()) +\
               "\n  ".join(["Priority {} with {} messages".format(k, v.size()) for k, v in self._priority_queues.items()])


class NetworkManager:
    def __init__(self, sleep_time=0.001, use_thread_pool=True, thread_pool_workers=10):
        self._queue = NetworkPriotyQueue()
        self._sender_thread = None
        self._running = False
        self._sleep_time = sleep_time
        self._thread_pool = None
        self._thread_pool_workers = thread_pool_workers
        self._use_thread_pool = use_thread_pool

    def __bool__(self):
        return self._running

    # Queues a message to be sent
    def post(self, priority: int, send_func, *args, **kwargs):
        self._queue.put(NetworkMessage(priority, send_func, *args, **kwargs))

    def has_messages(self):
        return not self._queue.empty()

    def is_running(self):
        return self._sender_thread and self._running

    def start(self):
        self._log("network manager start called")
        if self._sender_thread and self._running:
            self._log("sender already active")
            return

        if self._sender_thread or self._running:
            self._log("sender_thread and running mismatch")

        if self._use_thread_pool:
            self._thread_pool = ThreadPoolExecutor(max_workers=self._thread_pool_workers)
            
        self._sender_thread = threading.Thread(target=self._sender)
        self._running = True
        self._sender_thread.start()

    def stop(self):
        self._log("network manager stop called")
        if not self._sender_thread and not self._running:
            self._log("sender not running")
            return

        self._running = False
        if self._sender_thread:
            self._sender_thread.join()

        if self._use_thread_pool:
            self._thread_pool.shutdown()
            self._thread_pool = None


    # Get next message and return it
    def _get_next(self):
        return self._queue.get()
        
    def _sender(self):
        self._log("sender entering")
        while self._running:
            message = self._get_next()

            # If message is found, send it
            if message:
                if self._thread_pool:
                    self._thread_pool.submit(self._send, message)
                else:
                    threading.Thread(target=self._send, args=(message,)).start()

            # Otherwise, sleep before checking again
            else:
                sleep(self._sleep_time)
    
        self._log("sender exiting")

    def _send(self, message):
        message.send()

    def _log(self, message):
        # print(message)
        pass
    