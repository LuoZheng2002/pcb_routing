import socket
import json

HOST = "127.0.0.1"
PORT = 4000

def handle_connection(conn, addr):
    print(f"Connected by {addr}")
    while True:
        data = conn.recv(1024)
        if not data:
            print("no data")
            break
        data = data.decode('utf-8')
        data = json.loads(data)
        print(f"Received data: {data}")
        if data == "fetch_grid":
            reply = {
                "Ok":([
                    [(255, 0, 0), (0, 255, 0)],
                    [(0, 0, 255), (255, 255, 0)]
                ])
            }
            reply = json.dumps(reply)
            print(f"Sending reply: {reply}")
            conn.sendall(reply.encode())
        else:
            reply = {"error": "Unknown command"}
            conn.sendall((json.dumps(reply) + "\n").encode())
    conn.close()

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        print(f"Python server listening on {HOST}:{PORT}...")

        while True:
            conn, addr = s.accept()
            handle_connection(conn, addr)

if __name__ == "__main__":
    main()