import sys
sys.path.append("../protos")
sys.path.append("..")
import network_manager
import grpc
import services
import time

# Basic test to ensure that logging works as expected
def log_test(num_clients=2, num_messages=3, delay=0.01):
    print("Running log test")
    print("Number of clients:  ", num_clients)
    print("Messages per client:", num_messages)

    # Runs a simple test of the logging service
    server_host = "localhost"
    server_port = 13172

    # set up server
    log_server_nwm = network_manager.NetworkManager(host=server_host, port=server_port)
    log_server_service = services.LoggingServer()
    log_server_nwm.add_service(log_server_service)
    log_server_nwm.start()

    # set up clients
    clients = []
    for i in range(num_clients):
        log_client_nvm = network_manager.NetworkManager(port=server_port+i+1)
        log_client_nvm.add_service(services.LoggingClient(host=server_host, port=server_port))
        log_client_nvm.start()
        clients.append(log_client_nvm)

    for i in range(num_messages):
        for client in clients:
            srvc = client.get_service(services.LoggingClient.name)
            srvc.log("test category " + str(i%2), "test content " + str(i))
            time.sleep(delay)

    # Sleep for a time scaled by the number of clients and messages
    time.sleep(3 + 0.01 * num_clients * num_messages)
    log_server_nwm.stop()
    for client in clients:
        client.stop()
    time.sleep(1)

if __name__ == "__main__":
    log_test()