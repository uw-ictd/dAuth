import sys
sys.path.append("..")
import time
import queue
import network_manager
import random
import threading
from collections import deque

# THIS IS NOT A FORMAL TESTING FILE
# IT WAS USED TO TEST DURING DEVELOPMENT AND MAY NOT WORK

class TestQueue:
    def __init__(self):
        self.q = deque()
        self.time = 0
        self.count = 0

    def put(self, m):
        start = time.time()
        self.count += 1
        self.q.append(m)
        self.time = time.time() - start
    
    def get(self):
        if self.q:
            return self.q.popleft()
        else:
            return None
    
    def empty(self):
        return not self.q

class NetworkQueueTests:
    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    def run_basic_tests(self):
        self.test_functionality()
        self.test_performance()

    def run_performance_tests(self):
        self.test_performance(num_priorities=1, num_inserts=30000)
        self.test_performance(num_priorities=10, num_inserts=30000)
        self.test_performance(num_priorities=100, num_inserts=30000)
        self.test_performance(num_priorities=1000, num_inserts=30000)
        self.test_performance(num_priorities=10000, num_inserts=30000)
        self.test_performance(num_priorities=30000, num_inserts=30000)

    def run_stress_tests(self):
        self.test_stress(num_priorities=30000, num_inserts=30000, num_repeats=10)

    # Performs basic functionality test
    def test_functionality(self):
        print("-"*25)
        print("Starting functionality test")
        print()
        network_queue = network_manager.NetworkPriotyQueue()

        def test_send_func(priority, delay, i="i", j="j"):
            print(test_send_func.__name__, "test_send_func called: Priority: {}, Delay: {}s, i: {}, j: {}".format(priority, delay, i, j))
            time.sleep(delay)

        for i in range(10):
            for j in range(10):
                # Use j as priority
                message = network_manager.NetworkMessage(j, test_send_func, j, 0.01, i=i, j=j)
                network_queue.put(message)
        
        print(network_queue)
        
        while not network_queue.empty():
            message = network_queue.get()
            message.send()

        print("Functionality test finished")
        print("-"*25)

    # Compares the performance of generic queue with the priority queue
    # (it should be impossible for the priority queue to actually be better)
    def test_performance(self, num_priorities=10, num_inserts=10, message_delay=0):
        print("-"*25)
        print("Starting preformance test")
        print("  num_priorities:", num_priorities)
        print("  num_inserts:", num_inserts)
        print("  message_delay:", message_delay)
        print()

        def test_send_func(delay):
            time.sleep(delay)

        def add_all(l, q):
            for m in l:
                q.put(m)

        def comp(l):
            nw = network_manager.NetworkPriotyQueue()
            nwm = network_manager.NetworkPriotyQueue()
            q = TestQueue()

            q_insert_time = time.time()
            add_all(l, q)
            q_insert_time = time.time() - q_insert_time

            nw_insert_time = time.time()
            add_all(l, nw)
            nw_insert_time = time.time() - nw_insert_time

            nwm_insert_time = time.time()
            add_all(l, nwm)
            nwm_insert_time = time.time() - nwm_insert_time

            q_send_time = time.time()
            while not q.empty():
                message = q.get()
                message.send()
            q_send_time = time.time() - q_send_time

            nw_send_time = time.time()
            while not nw.empty():
                message = nw.get()
                message.send()
            nw_send_time = time.time() - nw_send_time

            nwm_send_time = time.time()
            while not nwm.empty():
                messages = nwm.get_multiple(10000000)
                for message in messages:
                    message.send()
            nwm_send_time = time.time() - nwm_send_time

            print(" Insert times (ms)")
            print("  Normal queue: (ms)", int(q_insert_time * 1000))
            print("  Network queue: (ms)", int(nw_insert_time * 1000))
            print("  Network M queue: (ms)", int(nwm_insert_time * 1000))
            print(" Send times")
            print("  Normal queue: (ms)", int(q_send_time * 1000))
            print("  Network queue: (ms)", int(nw_send_time * 1000))
            print("  Network M queue: (ms)", int(nwm_send_time * 1000))
            print()

        messages = []
        for i in range(num_inserts):
            messages.append(network_manager.NetworkMessage(i // (num_inserts // num_priorities), test_send_func, message_delay))
        print("Sorted test")
        comp(messages)

        messages = []
        for i in range(num_inserts):
            messages.append(network_manager.NetworkMessage(i % num_priorities, test_send_func, message_delay))
        print("Alternating test")
        comp(messages)

        messages = []
        for i in range(num_inserts):
            messages.append(network_manager.NetworkMessage(random.randint(0, num_priorities-1), test_send_func, message_delay))
        print("Random test")
        random.shuffle(messages)
        comp(messages)

        print()
        print("Performance test finsished")
        print("-"*25)

    def test_stress(self, num_priorities=30000, num_inserts=30000, num_repeats=3, message_delay=0):
        print("-"*25)
        print("Starting stress test")
        print("  num_priorities:", num_priorities)
        print("  num_inserts:", num_inserts)
        print("  num_repeats:", num_repeats)
        print("  message_delay:", message_delay)
        print()

        def test_send_func(delay):
            time.sleep(delay)

        def add_all(l, q):
            for m in l:
                q.put(m)

        def load(l, nw):
            nw_insert_time = time.time()
            add_all(l, nw)
            nw_insert_time = time.time() - nw_insert_time

            print("  Network queue insert times: (ms)", int(nw_insert_time * 1000))
        
        nw = network_manager.NetworkPriotyQueue()
        for t in range(num_repeats):
            messages = []
            for _ in range(num_inserts):
                messages.append(network_manager.NetworkMessage(random.randint(0, num_priorities-1), test_send_func, message_delay))
            print("load " + str(t) + " (" + str(len(nw._active_queues_heap))+ "/" + str(num_priorities) + " priorities used)")
            random.shuffle(messages)
            load(messages, nw)
        
        nw_send_time = time.time()
        while not nw.empty():
            # nw.get().send()
            messages = nw.get_multiple(300000000)  # wtf is this better than replacing 'messages'...
            for message in messages:
                message.send()
        nw_send_time = time.time() - nw_send_time
        print("Network queue send time: {} messages in {} ms (~{} m/s)".format(num_inserts*num_repeats, int(nw_send_time * 1000), int(num_inserts*num_repeats/nw_send_time)))

        print()
        print("Performance test finsished")
        print("-"*25)

        
class NetworkManagerTests:
    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    def test_functionality(self, iter_list):
        for iterations in iter_list:
            def test_func(priority, message):
                print(priority, ":", message)

            def run_test(nm):
                start = time.time()
                nm.start()

                for i in range(iterations):
                    nm.post(i%5, test_func, i%5, i)
                
                while nm.has_messages():
                    time.sleep(0.1)
                time.sleep(0.5)
                
                nm.stop()
                if not nm._use_thread_pool:
                    print("thread_only", "time taken", time.time() - start)
                else:
                    print("thread_pool_"+str(nm._thread_pool_workers), "time taken", time.time() - start)

            nm_thread_only = network_manager.NetworkManager(use_thread_pool=False)
            nm_thread_pool_10 = network_manager.NetworkManager(thread_pool_workers=10)
            nm_thread_pool_100 = network_manager.NetworkManager(thread_pool_workers=100)
            nm_thread_pool_1000 = network_manager.NetworkManager(thread_pool_workers=1000)

            managers = [nm_thread_only, nm_thread_pool_10, nm_thread_pool_100, nm_thread_pool_1000]

            for manager in managers:
                run_test(manager)

    def test_straight_performance(self, iter_list, sleep_time):
        for iterations in iter_list:
            print("Running performance with", iterations, "iterations")
            def test_func(priority, message):
                time.sleep(sleep_time)

            def run_test(nm):
                start = time.time()
                nm.start()

                for i in range(iterations):
                    nm.post(i%10, test_func, i%10, i)
                
                while nm.has_messages():
                    time.sleep(0.1)
                time.sleep(0.5)
                
                nm.stop()
                if not nm._use_thread_pool:
                    print(" thread_only", "time taken", time.time() - start)
                else:
                    print(" thread_pool_"+str(nm._thread_pool_workers), "time taken", time.time() - start)

            nm_thread_only = network_manager.NetworkManager(use_thread_pool=False)
            nm_thread_pool_10 = network_manager.NetworkManager(thread_pool_workers=10)
            nm_thread_pool_100 = network_manager.NetworkManager(thread_pool_workers=100)
            nm_thread_pool_1000 = network_manager.NetworkManager(thread_pool_workers=1000)

            managers = [nm_thread_only, nm_thread_pool_10, nm_thread_pool_100, nm_thread_pool_1000]

            for manager in managers:
                run_test(manager)
            
            print()

    def test_performance(self, iter_list, priorities, rate, message_per_rate):
        for iterations in iter_list:
            print("Running performance with", iterations, "iterations")
            delays = {}

            def test_func(priority, message):
                delays[priority].append(time.time() - message)
                t = time.time()
                i = 0
                while time.time() < t + 0.001:
                    i += 1
                

            def sender(nm):
                for i in range(iterations):
                    if i % message_per_rate == 0:
                        time.sleep(rate)
                    nm.post(i%priorities, test_func, i%priorities, time.time())

            def run_test(nm):
                for i in range(priorities):
                    delays[i] = []

                nm.start()

                t = threading.Thread(target=sender, args=(nm,))
                t.start()
                
                start = time.time()
                t.join()  # go till messages stop sending, then till messages through queue
                while nm.has_messages():
                    time.sleep(0.01)
                
                while(sum([len(i) for i in delays.values()]) < iterations):
                    time.sleep(0.01)

                if not nm._use_thread_pool:
                    print(" thread_only", "time taken", time.time() - start)
                else:
                    print(" thread_pool_"+str(nm._thread_pool_workers), "time taken", time.time() - start)

                avg = {}
                for priority, vals in delays.items():
                    avg[priority] = (sum(vals) * 1000) // len(vals)

                print(avg)
                # print("  Priority", priority, "with avg delay", (sum(vals) * 1000) // len(vals), "ms")
    
                nm.stop()

            nm_thread_only = network_manager.NetworkManager(use_thread_pool=False)
            nm_thread_pool_10 = network_manager.NetworkManager(thread_pool_workers=10)
            nm_thread_pool_100 = network_manager.NetworkManager(thread_pool_workers=100)
            nm_thread_pool_1000 = network_manager.NetworkManager(thread_pool_workers=1000)

            managers = [nm_thread_only, nm_thread_pool_10, nm_thread_pool_100, nm_thread_pool_1000]

            for manager in managers:
                run_test(manager)
            
            print()

if __name__ == "__main__":
    nqt = NetworkQueueTests()
    # nqt.test_functionality()
    nqt.run_performance_tests()
    # nqt.run_stress_tests()
    nmt = NetworkManagerTests()
    # nmt.test_functionality([10])
    # nmt.test_straight_performance([10000, 100000], 0.000001)
    # nmt.test_straight_performance([10000, 100000], 0.1)
    # nmt.test_performance([10], 5, 0.01, 10)
    # nmt.test_performance([10000], 10, 0.01, 10)