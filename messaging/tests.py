import time
import texttable
from messaging_client import MessagingClient, GlobalMessagingClient, GlobalLoggingClient

# Builds a 
def test_client(num_messages, stream_max, stream_max_wait):
    start = time.time()
    GlobalMessagingClient.set_client(MessagingClient(stream_max=stream_max, stream_max_wait=stream_max_wait))
    GlobalMessagingClient.start()

    # Set up wrapper class
    tag = "<M:" + str(num_messages) + ",SM:" + str(stream_max) + ",SMW:" + str(stream_max_wait) + ">"
    GlobalLoggingClient.set_host("Test_Client|" + tag)
    log = GlobalLoggingClient.log

    # Send test messages
    for i in range(num_messages):
        log("category" + str(i % 3), "content " + str(i))

    while GlobalLoggingClient.messages_left() > 0:
        time.sleep(0.1)

    GlobalMessagingClient.stop()

    return time.time() - start

def run_test(num_message_set, stream_max_set, iterations):
    print("-"*50)
    print("Running test for logging service")
    num_message_set = list(set(num_message_set))
    num_message_set.sort()
    stream_max_set = list(set(stream_max_set))
    stream_max_set.sort()

    results = {}
    for num_messages in num_message_set:
        results[num_messages] = {}

        print(" Running tests with", num_messages, "messages")
        for stream_max in stream_max_set:
            results[num_messages][stream_max] = {}
            
            # results[num_messages][stream_max][True] =[]
            results[num_messages][stream_max][False] = []

            for _ in range(iterations):
                results[num_messages][stream_max][False].append(test_client(num_messages, stream_max, False))
                # results[num_messages][stream_max][True].append(test_client(num_messages, stream_max, True))
    
    print(" Results (averaged over", iterations, "iterations):")
    result_table = []
    result_table.append(["Messages Sent", "Stream Max", "Stream Max Wait", "Total Time (s)", "Rate (M/s)"])
    for num_messages, smresults in results.items():
        for stream_max, smwresults in smresults.items():
            for stream_max_wait, result_list in smwresults.items():
                avg = sum(result_list) / len(result_list)
                result_table.append([str(num_messages),str(stream_max), str(stream_max_wait), str(avg), str(num_messages / avg)])
    
    table = texttable.Texttable()
    table.add_rows(result_table)
    table_str = table.draw()
    print(table_str)

    print("-"*50)

num_message_set = [1000, 10000, 100000]
stream_max_set = [100, 1000, 10000, 100000]
iterations = 3
run_test(num_message_set, stream_max_set, iterations)
exit(0)