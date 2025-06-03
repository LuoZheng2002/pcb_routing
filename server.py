import socket
import json
from datatypes import Color, ColorGrid
from dataclasses import asdict

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
            # Create some Color instances
            red = Color(255, 0, 0)
            green = Color(0, 255, 0)
            blue = Color(0, 0, 255)
            # Create a 2x2 grid
            grid_data = [
                [red, green],
                [blue, red]
            ]
            # Create the ColorGrid instance
            color_grid = ColorGrid(grid=grid_data)
            reply = json.dumps(asdict(color_grid))
            print(f"Sending reply: {reply}")
            conn.sendall(reply.encode())
        else:
            print(f"Unknown command: {data}")
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