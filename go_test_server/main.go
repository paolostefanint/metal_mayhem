/*package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
)

type Player struct {
	ID          int       `json:"id"`
	P           []float32 `json:"p"`
	Dir         string    `json:"dir"`
	Attack      bool      `json:"attack"`
	Health      float64   `json:"health"`
	SpriteState string    `json:"sprite_state"`
}

type GameState struct {
	CurrentTime  float64  `json:"current_time"`
	CurrentState string   `json:"current_state"`
	Players      []Player `json:"players"`
}

var connections = make(map[*websocket.Conn]bool)
var broadcast = make(chan string)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

func main() {
	http.HandleFunc("/", handleWebSocket)
	sendDataToClients()
	log.Fatal(http.ListenAndServe(":42000", nil))
}

func handleWebSocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println(err)
		return
	}
	defer conn.Close()

	connections[conn] = true

	for {
		// Keep the connection alive
	}
}

func sendDataToClients() {
	dataFile, err := ioutil.ReadFile("data.json")
	if err != nil {
		log.Fatal(err)
	}

	var dataArray []GameState
	err = json.Unmarshal(dataFile, &dataArray)
	if err != nil {
		log.Fatal(err)
	}

	for {
		for _, gameState := range dataArray {
			out, err := json.Marshal(gameState)
			if err != nil {
				log.Fatal(err)
			}
			sendToAllClients(string(out))
			log.Println("Sent data to clients", string(out))
			time.Sleep(time.Second)
		}
		time.Sleep(time.Second)
	}
}

func sendToAllClients(message string) {
	for conn := range connections {
		conn.WriteMessage(websocket.TextMessage, []byte(message))
		time.Sleep(time.Second)
	}
}

func init() {
	go sendDataToClients()
}



--*/
package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"time"

	guuid "github.com/google/uuid"
	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	log "github.com/sirupsen/logrus"
)

type Player struct {
	ID          int       `json:"id"`
	P           []float32 `json:"p"`
	Dir         string    `json:"dir"`
	Attack      bool      `json:"attack"`
	Health      float64   `json:"health"`
	SpriteState string    `json:"sprite_state"`
}

type GameState struct {
	CurrentTime  float64  `json:"current_time"`
	CurrentState string   `json:"current_state"`
	Players      []Player `json:"players"`
}

type CustomConn struct {
	Conn *websocket.Conn
	ID   guuid.UUID
}

var connections map[guuid.UUID]*CustomConn

func main() {
	enablelog := false
	if len(os.Args) > 1 && os.Args[1] == "log" {
		enablelog = true
	}
	l := &log.Logger{}
	connections = make(map[guuid.UUID]*CustomConn)
	//server
	r := mux.NewRouter()
	srv := &http.Server{
		Addr:         ":42000",
		WriteTimeout: time.Second * 15,
		ReadTimeout:  time.Second * 15,
		IdleTimeout:  time.Second * 60,
		Handler:      r, // Pass our instance of gorilla/mux in.
	}

	r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		Connect(w, r, l)
	})

	go func() {
		Execute(enablelog)
	}()

	fmt.Printf("start listening on %s\n", srv.Addr)
	l.Fatal(srv.ListenAndServe())
}

func Connect(w http.ResponseWriter, r *http.Request, l *log.Logger) {
	var upgrader = websocket.Upgrader{
		ReadBufferSize:    4096,
		WriteBufferSize:   4096,
		EnableCompression: true,
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		l.Fatal(err)
		return
	}
	g := guuid.New()
	cc := CustomConn{
		ID:   g,
		Conn: c,
	}
	connections[g] = &cc
	fmt.Printf("incoming connection %s from %s\n", g.String(), cc.Conn.LocalAddr().String())
	defer func(conn *websocket.Conn, g guuid.UUID) {
		delete(connections, g)
		c.Close()
	}(c, g)
	// mantiene la connessione aperta
	for {

	}
}

func Execute(enablelog bool) {
	fmt.Println("executing")
	dataFile, err := ioutil.ReadFile("data.json")
	if err != nil {
		fmt.Println(err.Error())
	}

	var dataArray []GameState
	err = json.Unmarshal(dataFile, &dataArray)
	if err != nil {
		fmt.Println(err.Error())
	}

	for {
		//if no connection wait and try again
		if len(connections) == 0 {
			fmt.Println("no connections")
			time.Sleep(1 * time.Second)
			continue
		}
		for _, gameState := range dataArray {
			for _, c := range connections {

				msg, err := json.Marshal(gameState)
				if err != nil {
					fmt.Println(err.Error())
				}

				//msg := "send message to conn " + u.String()
				err = c.Conn.WriteMessage(websocket.TextMessage, []byte(msg))
				if err != nil {
					fmt.Println(err.Error())
				}
				if enablelog {
					fmt.Printf("%v\n", string(msg))
				}
			}

			time.Sleep(time.Second)
		}
		time.Sleep(10 * time.Millisecond)
	}
}
