## Broker network protocol

```
LSB                                                                                                                                         MSB
  1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8   1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8
+-----------------+-----------------+----------------------------------+----------------------------------+----------------------------------+
|     Version     |      Type       |            Message ID            |             Length               |      16-bit Checksum CRC-16      |
+-----------------+-----------------+----------------------------------+----------------------------------+----------------------------------+
|                                                           Payload = Length bytes long                                                      |
+--------------------------------------------------------------------------------------------------------------------------------------------+
```

**Info**

- Big Endian
- CON: Maximum number of messages at any one time = 65535 (2^16 - 1)
- Header Size = 8 bytes (64 bits)
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