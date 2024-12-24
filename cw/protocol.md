## Broker network protocol

```
LSB                                                                                                       MSB
  1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8  
+-----------------+-----------------+----------------------------------+----------------------------------+
|     Version     |      Type       |            Message ID            |            Image Size            | 
+-----------------+-----------------+----------------------------------+----------------------------------+
+----------------------------------------------------+----------------------------------+
|                       Length                       |      16-bit Checksum CRC-16      |
+----------------------------------------------------+----------------------------------+
+---------------------------------------------------------------------------------------------------------+
|                                                           Payload = Length bytes long                   |
+---------------------------------------------------------------------------------------------------------+
```

**Info**

- Big Endian
- CON: Maximum number of messages at any one time = 65535 (2^16 - 1)
- Header Size = 11 bytes 
- Maximum payload size = 32768 bytes (2^15)
- Maximum message size = 32776 bytes (2^15 + 8)

**Brokers**

- Broker pool uses Virtual IP address for pool to allow single access point
- 1 leader (active) broker will hold the VIP and receive and delegate requests to worker pool via worker with lowest load
- 1 Failover broker (from worker pool) will take over if leader fails via leader election. Failover broker will be determined from worker pool by quickest response time (easy to implement)
    - Failover will be detected via heartbeat messages
    - Leader election will then be doine via a simplified raft algorithm
    - New leader will then take over VIP

1 000 000 000 1 000 000 000 1 

- Storing as arrays of uint32 = 32768 bytes total 
- Storing as map of bits =

start                                                                                                                                                       end 
 000 000 000 000 000 000    000 000 000 000 000 000    000 000 000 000 000 000   000 000 000 000 000 000  .... 000 000 000 000 000 000    

Start = S
End = E

struct cell {
    neigbours: []&cell
}

cells = map[uint32]cell
index = 0
previousIndex = 0
numberOfCells = 0
while S < E {
    i, previousCell := cells[numberOfCells-1]
    xy := S[index:index+20]
    if cell := cells[xy]; cell == nil {
        cell = cell{}
        cells[xy] = cell
    }

    if isNeighbour(i, xy) {
        i.neighbours = append(i.neighbours, &cell)
        cell.neighbours = append(cell.neighbours, &previousCell)
    }

    previousIndex = index
    index = index + 20
    numberOfCells++
}



0001 0001
0001 0000  or = x or y = valid




func isNeighbour(xy1, xy2 uint32) bool {
    bitOp := xy1 | xy2
    if bitOp == xy1 || bitOp == xy2 {
        return true
    }
}


V1 to v2: 5
V6 to v1: 4
V1 to v5: 7
v2 to v7: 2
V3 to v2: 8
v3 to v8: 10
v8 to V3: 5
v4 to V3: 2
v4 to v9:2
v9 to V4:1
v4 to v5:5
V5 to v10:3
v6 to v8:2
v9 to V6:10
v7 to v9:10
v1 to V70:6
V8 to v10:6

import React, { useState } from "react";
import { View, TextInput, Button, Text, StyleSheet, TouchableOpacity } from "react-native";

export default function LoginPage({ navigation }) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <View style={styles.container}>
      <View style={styles.imagePlaceholder} />

      <TextInput
        style={styles.input}
        placeholder="username"
        value={username}
        onChangeText={setUsername}
      />
   
      <TextInput
        style={styles.input}
        placeholder="Password"
        value={password}
        secureTextEntry={true}
        onChangeText={setPassword}
      />
   
      <TouchableOpacity 
        style={styles.forgotPassword} 
        onPress={() => navigation.navigate("ResetPassword")}>
        <Text style={styles.forgotPasswordText}>forgot password?</Text>
      </TouchableOpacity>
   
      <TouchableOpacity style={styles.loginButton}>
        <Text style={styles.loginButtonText}>Log In</Text>
      </TouchableOpacity>

      <Text style={styles.signupText}>Don't have an account, sign up now.</Text>
      <View style={styles.linkContainer}>
        <TouchableOpacity onPress={() => navigation.navigate("RegisterUser")}>
          <Text style={styles.link}>attend an event</Text>
        </TouchableOpacity>
        <Text style={styles.divider}> / </Text>
        <TouchableOpacity onPress={() => navigation.navigate("RegisterHost")}>
          <Text style={styles.link}>host an event</Text>
        </TouchableOpacity>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    justifyContent: "center",
    backgroundColor: "#fff",
    marginTop: -100, 
  },
  imagePlaceholder: {
    width: 100,
    height: 100,
    backgroundColor: "#e0e0e0",
    alignSelf: "center",
    marginBottom: 120, 
  },
  input: {
    height: 40,
    borderColor: "gray",
    borderWidth: 1,
    borderRadius: 10, 
    marginBottom: 10,
    padding: 10,
  },
  forgotPassword: {
    alignItems: "flex-end",
    marginBottom: 20,
  },
  forgotPasswordText: {
    color: "black",
    fontSize: 12,
  },
  loginButton: {
    backgroundColor: "#5DC268",
    borderRadius: 10,
    paddingVertical: 10,
    marginBottom: 20,
    alignItems: "center",
  },
  loginButtonText: {
    color: "#fff",
    fontSize: 18,
    fontWeight: "bold",
  },
  signupText: {
    textAlign: "center",
    marginBottom: 10,
  },
  linkContainer: {
    flexDirection: "row",
    justifyContent: "center",
  },
  link: {
    color: "#0000FF",
    fontWeight: "bold",
  },
  divider: {
    marginHorizontal: 5,
  },
});


+----------+                                 +---------------+                          +-------+  
|Event Host|   ------|bookedVenue|------>    | Venue Booking |  <------|heldAt|------   | Event |
+----------+                                 +---------------+                          +-------+
                                                     ^
                                                     |
                                                     |
                                                 |booking|
                                                     |
                                                     |
                                                     |
                                                 +-------+   
                                                 | Venue |
                                                 +-------+




To ensure that your code waits for the corresponding response and not just any message from the TCP stream, you need to implement a mechanism that matches each response to its original request. This is commonly achieved by assigning a unique message ID to each request and using it to correlate the response.

Here's how you can implement this:

Assign a Unique Message ID: Before sending a request, generate a unique message ID and include it in the request header.
Create a Pending Requests Map: Use a shared, thread-safe data structure (like Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseType>>>>) to keep track of pending requests. The key is the message ID, and the value is a channel sender that will be used to deliver the response.
Send the Request and Wait for the Response:
Insert the message ID and the channel sender into the pending requests map.
Send the request over the TCP stream.
Wait for the response by awaiting the receiver end of the channel.
Read Responses in a Background Task:
Start a background task that continuously reads from the TCP stream.
When a response is received, extract the message ID.
Use the message ID to find the corresponding channel sender in the pending requests map.
Send the response through the channel, allowing the original task to resume.
Here's a concrete example incorporating these steps:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, oneshot};
use futures::StreamExt;

type ResponseType = IndexSet<u32>;
type ErrorType = Box<dyn std::error::Error + Send + Sync>;

struct Client {
    stream: TcpStream,
    pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseType>>>>,
    msg_id_counter: Arc<Mutex<u32>>,
}

impl Client {
    async fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            msg_id_counter: Arc::new(Mutex::new(0)),
        }
    }

    async fn get_next_msg_id(&self) -> u32 {
        let mut counter = self.msg_id_counter.lock().await;
        *counter += 1;
        *counter
    }

    async fn send_request(
        &self,
        params: PacketParams,
        request_data: &[u8],
    ) -> Result<ResponseType, ErrorType> {
        let msg_id = params.msg_id;

        // Create a oneshot channel
        let (tx, rx) = oneshot::channel();

        // Insert the sender into the pending_requests map
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(msg_id, tx);
        }

        // Send the request over the stream
        self.stream.write_all(request_data).await?;

        // Await the response
        match rx.await {
            Ok(response) => Ok(response),
            Err(_) => Err("Response channel closed unexpectedly".into()),
        }
    }

    async fn start_response_reader(self: Arc<Self>) {
        let mut reader = tokio::io::BufReader::new(&self.stream);
        loop {
            // Read and parse the response
            let mut buffer = vec![0; 1024];
            let n = reader.read(&mut buffer).await.unwrap();
            if n == 0 {
                break; // Connection closed
            }

            // Extract msg_id and response_data from buffer
            let (msg_id, response_data) = parse_response(&buffer[..n]);

            // Find the pending request and send the response
            let tx_opt = {
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&msg_id)
            };

            if let Some(tx) = tx_opt {
                let _ = tx.send(response_data);
            } else {
                // Handle unknown msg_id
            }
        }
    }
}

// Usage in your worker function
async fn worker_process_slice_request(
    client: Arc<Client>,
    args: ProcessSliceArgs,
    response_channel: Sender<IndexSet<u32>>,
) -> Result<(), ErrorType> {
    let msg_id = client.get_next_msg_id().await;
    let params = PacketParams {
        fn_call_id: FunctionCall::PROCESS_SLICE,
        msg_id,
        image_size: args.params.image_width as u16,
    };

    let packet = Packet::new();
    let request_data = packet.encode(params, &IndexSet::new())?;

    // Send the request and wait for the response
    match client.send_request(params, &request_data).await {
        Ok(response_data) => {
            response_channel.send(response_data).await?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

```
In this example:

Client Structure: Holds the TCP stream and a map of pending requests.
send_request Method: Sends the request and waits for the corresponding response using a oneshot channel.
start_response_reader Method: Continuously reads from the TCP stream and dispatches responses to the appropriate request handlers.
worker_process_slice_request Function: Generates a unique message ID, sends the request, and waits for the response.
Key Points:

Unique Message IDs: Ensure that each request can be matched to its response.
Pending Requests Map: Allows the background reader to deliver responses to the correct awaiting tasks.
Asynchronous Communication: Utilizes async/await and channels to handle concurrent operations without blocking.
Benefits of This Approach:

Concurrency: Multiple requests can be sent and awaited concurrently without interference.
Scalability: The system can handle a high volume of requests and responses efficiently.
Robustness: Proper error handling and cleanup mechanisms can be added to handle network failures or unexpected conditions.
By implementing this mechanism, you ensure that your code waits for the specific response corresponding to each request, maintaining the integrity and reliability of your RPC implementation.