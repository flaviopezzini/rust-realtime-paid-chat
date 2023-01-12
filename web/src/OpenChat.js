import './OpenChat.css';

import React, { useState } from 'react';

import useWebSocket from 'react-use-websocket';

function OpenChat(props) {

  const [messageHistory, setMessageHistory] = useState('');
  const [newMessageText, setNewMessageText] = useState('');

  const socketUrl = 'ws://localhost:9000/websocket';

  const {
    sendMessage,
    sendJsonMessage,
    lastMessage,
    lastJsonMessage,
    readyState,
    getWebSocket,
  } = useWebSocket(socketUrl, {
    onOpen: () => {
      console.log('opened');
      sendMessage(props.username);
    },
    onClose: () => {
        console.log("closed");
    },
    onMessage: (e) => {
      console.log('event: ' + JSON.stringify(e));
      setMessageHistory((prev) => prev.concat(e.data+"\r\n"));
    },
    //Will attempt to reconnect on all close events, such as server shutting down
    shouldReconnect: (closeEvent) => true,
  });

  var newMessageOnChange = function(e) {
    var key = e.nativeEvent.data;
    console.log(key);
    if (key === "Enter") {
      console.log("New message is : " + newMessageText);
      sendMessage(newMessageText);
      setNewMessageText((prev) => "");
    } else {
      setNewMessageText((prev) => prev + key)
    }
  }

  return (
    <div className="OpenChat">
      <header className="OpenChat-header">
        <textarea id="chat" className="chat-area" cols="30" rows="10"
          value={messageHistory}
          onChange={(e) => setMessageHistory(e.target.value)}
        ></textarea>
        <input id="input" className="new-message-input" type="text" placeholder="chat"
           value={newMessageText}
           onChange={newMessageOnChange}
         />
      </header>
    </div>
  );
}

export default OpenChat;