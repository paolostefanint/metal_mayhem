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

type GameState struct {
	Game struct {
		Status         string `json:"status"`
		Time           int64  `json:"time"`
		RemainingTime  int    `json:"remainingTime"`
		RoundCountdown int    `json:"roundCountdown"`
	} `json:"game"`
	Players []struct {
		ID        int    `json:"id"`
		Connected bool   `json:"connected"`
		Name      string `json:"name"`
		SessionID string `json:"sessionId"`
		Sub       string `json:"sub"`
		Color     string `json:"color"`
		Pic       string `json:"pic"`
		Avatar    string `json:"avatar"`
		Life      int    `json:"life"`
		Position  struct {
			X float64 `json:"x"`
			Y float64 `json:"y"`
		} `json:"position"`
		Direction   string `json:"direction"`
		SpriteState string `json:"spriteState"`
	} `json:"players"`
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

				err = c.Conn.WriteMessage(websocket.TextMessage, []byte(msg))
				if err != nil {
					fmt.Println(err.Error())
					connections[c.ID].Conn.Close()
					delete(connections, c.ID)
					break
				}
				if enablelog {
					fmt.Printf("%v\n", string(msg))
				}
			}

			time.Sleep(100 * time.Millisecond)
		}
		//time.Sleep(10 * time.Millisecond)
	}
}
