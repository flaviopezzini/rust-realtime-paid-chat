import logo from './logo.svg';
import './Chat.css';

import React, { useState, useCallback, useEffect } from 'react';

import useWebSocket from 'react-use-websocket';

function Chat() {

  const [username, setUsername] = useState('');
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
      sendMessage(username);
    },
    onMessage: (e) => {
      console.log('event: ' + JSON.stringify(e));
      setMessageHistory((prev) => prev.concat(e.data+"\r\n"));
    },
    //Will attempt to reconnect on all close events, such as server shutting down
    shouldReconnect: (closeEvent) => true,
  });

  let newMessageOnBlur = function(e) {
    if (e.key == "Enter") {
      sendMessage(newMessageText);
      setNewMessageText()
      setNewMessageText((prev) => "");
    }
  }

  return (
    <div className="Chat">
      <header className="Chat-header">
        <input id="username" className="user-name" type="text" placeholder="username"
          value={username} defaultValue="" />
        <button id="join-chat" type="button">Join Chat</button>
        <textarea id="chat" className="chat-area" cols="30" rows="10"
          value={messageHistory}
        ></textarea>
        <input id="input" className="new-message-input" type="text" placeholder="chat"
           onBlur={newMessageOnBlur} value={newMessageText} />
      </header>
    </div>
  );
}

export default Chat;