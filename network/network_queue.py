from threading import Lock
from collections import deque
import heapq


# Small class that holds on to a priority, send function, and message
class NetworkMessage:
    # Takes in the priorty, sending function, and args to the sending function
    # Also allows for a store function and list of args to pass to the function
    def __init__(self, priority, send_function, message, is_stream=False,
                 store_func=None, store_args=None):
        self.priority = priority
        self._send_function = send_function
        self._message = message
        self._is_stream = is_stream
        self._store_func = store_func
        self._store_args = store_args

        # Precompute size
        try:
            if self._is_stream:
                self._size = sum([m.ByteSize() for m in self._message])
            else:
                self._size = self._message.ByteSize()
        except:
            self._size = None

    # Runs the sending function with the given data
    def send(self):
        if self._is_stream:
            res = self._send_function(iter(self._message))
        else:
            res = self._send_function(self._message)
        
        if self._store_func:
            self._store_func(res, store_args=self._store_args)

    # Returns the size, in bytes, of the message
    def size(self):
        return self._size


# Wrapper class for a single priority queue unit
# Implemenets a comparable function for sorting
class QueueUnit:
    def __init__(self, priority, comparator=None):
        self.priority = priority
        self._queue = deque()
        self._comparator = comparator

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
        if self._comparator:
            return self._comparator(self, other)
        return self.priority < other.priority

    def __str__(self):
        return "QueueUnit: " + str(self.priority)


# Priority queue intended to store NetworkMessages, lower values have higher priority
# Any interger can be used as a priority, implemeneted by mapping priorities to queues
# Thread safe
class NetworkPriotyQueue:
    # Params:
    # - known_priorities: a list of known priorities to be initialized at first
    # - use_only_known: if known_priorities is not empty/none, don't allow new priorities
    def __init__(self, known_priorities=None, use_only_known=False):
        # mapping of all priority queues
        self._priority_queues = {}

        # holds all the queues with available messages
        self._active_queues_heap = []

        # remaining messages
        self._qsize = 0

        # lock for critical areas
        self._lock = Lock()

        # create any known priorities
        if known_priorities:
            for priority in known_priorities:
                self._priority_queues[priority] = QueueUnit(priority)

        self._use_only_known = known_priorities and use_only_known

    # Appends a new message to the appropriate queue
    def put(self, message: NetworkMessage):
        self._lock.acquire()

        # create a queue if one doesn't exists, unless queue is marked as known only
        if not message.priority in self._priority_queues:
            # raise exception for unknown priority
            if self._use_only_known:
                self._lock.release()
                raise KeyError("priority does not exist, queue using only knwon priorities")

            # create queue
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

