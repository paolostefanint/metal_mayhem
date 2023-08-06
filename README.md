# MetalMayhem

MetalMayhem è un picchiaduro in tempo reale nello stile dei giochi arcade anni 90 come Final Fight, Double Dragon, Golden Axe, etc.

I giocatori scelgono il personaggio tra tre possibili scegliendo un colore che li identifica da applicare a dei dettagli della sprite del personaggio.

I giocatori si muovono in un arena e si scontrano tra di loro, l'obiettivo è quello di eliminare tutti gli avversari.
Chi rimane vivo vince.

Alla fine della partita viene mostrata la classifica dei giocatori.

## Architettura
Il server è scritto in rust e gestisce la logica del gioco, la comunicazione con i client e la persistenza dei dati.

Il client condiviso dell'area di lotta è fatto in unity, verrà proiettato su un muro per essere visibile da tutti i giocatori.

I giocatori controlleranno i loro personaggi con un applicazione web da loro telefono che mostra un joysitck ad 8 direzioni e un pulsante attacco

Il gioco è per 8 persone contemporaneamente.

Fasi di gioco:
* Aggiunta giocatori
* Selezione personaggio
* Selezione colore
* Gioco
* Classifica

## Selezione personaggio
* I giocatori indicano il loro nome.
* I giocatori scelgono accedono alla room della partita, scegliendo il personaggio tra tre possibili con caratteristiche diverse.
* I giocatori selezionano un colore che li identifichi, che verrà a applicato a dei dettagli della sprite del personaggio.

**Personaggio 1**:
* velocità di movimento alta
* attacco medio
* vita bassa

**Personaggio 2**:
* velocità di movimento media
* attacco medio
* vita media

**Personaggio 3**:
* velocità di movimento bassa
* attacco alto
* vita alta

## Gioco
I client web inviano i messaggi al server

il server invia i messaggi ai client unity con lo stato del gioco.
(./example/client_message.json)[Esempio di messaggio inviato dal server al client]

