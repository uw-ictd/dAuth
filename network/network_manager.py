from threading import Lock
from time import time, sleep
from collections import deque
from concurrent.futures import ThreadPoolExecutor
import heapq
import threading

# Possible improvements/directions:
#  - Add a 'smart' way of keeping QUs in the active queue, maybe specify frequent priorities?
#  - Add a param to pop multiple messages at a time (avoid overhead of getting active queue)
#  - Add bandwidth limiter (lots of problems with this)
#  -- maybe make it specific to certain queues?
#  - flip comparator (or add init comparator?) for QueueUnit
#  - Maybe add some way to store results? A dict perhaps?
#  -- May be too much trouble, just assume the user passes a storage object

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

    def __bool__(self):
        return self._queue
    
    def __lt__(self, other):
        return self.priority < other.priority

    def __str__(self):
        return "QueueUnit: " + str(self.priority)


# Priority queue intended to store NetworkMessages, lower values have higher priority
# Any interger can be used as a priority, implemeneted by mapping priorities to queues
# Thread safe
class NetworkPriotyQueue:
    def __init__(self):
        # mapping of all priority queues
        self._priority_queues = {}

        # ordered list of priority queue keys
        self._priority_queue_keys = []

        # holds all the queues with available messages
        self._active_queues_heap = []

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
            self._insert_active_queue(qu)

        # put the message in the queue and increment total
        qu.put(message)
        self._qsize += 1

        self._lock.release()
    
    # Finds and returns the next highest priority message, or None if not found
    def get(self):
        return self._next()

    # Returns at most num_messages (less if there aren't that many left)
    def get_multiple(self, num_messages):
        return self._next(num_messages=num_messages)
    
    def empty(self):
        return self._qsize < 1
    
    def size(self):
        return self._qsize
    
    def _next(self, num_messages=None):
        # check that at least one message exists
        if self.empty():
            return None

        self._lock.acquire()

        # sanity check
        if len(self._active_queues_heap) < 1:
            self._lock.release()
            return None
        
        # get up to num_messages messages
        if num_messages is not None:
            num_messages = min(self._qsize, num_messages)
            message = []
            while num_messages > 0:
                # grab the next queue in line and figure number of messages to get from it
                qu = self._get_highest_active_queue()
                num_get = min(qu.size(), num_messages)
                num_messages -= num_get
                target = self._qsize - num_get

                while self._qsize > target:
                    message.append(qu.get())
                    self._qsize -= 1

                # remove queue if empty
                if qu.empty():
                    self._remove_highest_active_queue()

        # get the highest priority queue and pop a message from it
        else:
            qu = self._get_highest_active_queue()
            message = qu.get()
            self._qsize -= 1

            # remove queue from list if it has no messages left
            if qu.empty():
                self._remove_highest_active_queue()

        self._lock.release()

        return message

    # Add a new priority queue to the active list
    def _insert_active_queue(self, q):
        heapq.heappush(self._active_queues_heap, q)
    
    # Return the highest priority queue
    def _get_highest_active_queue(self):
        if len(self._active_queues_heap) > 0:
            # in a heap, the first element is the highest priority
            return self._active_queues_heap[0]
        else:
            return None

    # Simply remove the highest priority queue
    def _remove_highest_active_queue(self):
        if len(self._active_queues_heap) > 0:
            heapq.heappop(self._active_queues_heap)

    def __str__(self):
        return "Network Prioirty Queue ({} total messages)\n  ".format(self.size()) +\
               "\n  ".join(["Priority {} with {} messages".format(k, v.size()) for k, v in self._priority_queues.items()])


# Maintains a queue of network messages at certain priorities
# Sends messages in order of priority as it gets them
class NetworkManager:
    # Params:
    # - block_size: Number of messages to pull from the queue at a time, or <1 to pull all messages
    #               higher numbers can be more efficient, but may not work well if priorities must be followed closely
    # - sleep_time: delay before checking for new messages (higher results in less cpu usage, but higher potential latency)
    # - use_thread_pool: uses a thread pool instead of spawing threads for each message. Less overhead and more efficient in most cases
    # - thread_pool_workers: Number of workers for the thread pool, ignored if no thread pool is used
    def __init__(self, block_size=1, sleep_time=0.0001, use_thread_pool=True, thread_pool_workers=10):
        self._queue = NetworkPriotyQueue()
        self._sender_thread = None
        self._running = False
        self._sleep_time = sleep_time
        self._thread_pool = None
        self._thread_pool_workers = thread_pool_workers
        self._use_thread_pool = use_thread_pool
        self._block_size = block_size

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

    # Get next message(s), return as list
    def _get_next(self):
        if self._block_size == 1:
            return [self._queue.get()]
        else:
            return self._queue.get_multiple(self._block_size if self._block_size > 1 else self._queue.size())
        
    def _sender(self):
        self._log("sender entering")
        while self._running:
            # get a list of avaiable messages
            messages = self._get_next()

            # If message is found, send it
            if messages:
                if self._thread_pool:
                    self._thread_pool.map(self._send, messages)
                else:
                    for message in messages:
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
    