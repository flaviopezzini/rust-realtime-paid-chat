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

  const handleChange = (event) => {
    setNewMessageText(event.target.value);
  };

  const handleKeyDown = (event) => {
    if (event.key === 'Enter') {
      console.log("New message is : " + newMessageText);
      sendMessage(newMessageText);
      setNewMessageText((prev) => "");
    }
  };

  return (
    <div className="OpenChat">
      <header className="OpenChat-header">
        <textarea id="chat" className="chat-area" cols="30" rows="10"
          value={messageHistory}
          onChange={(e) => setMessageHistory(e.target.value)}
        ></textarea>
        <input id="input" className="new-message-input" type="text" placeholder="chat"
           value={newMessageText}
           onChange={handleChange}
           onKeyDown={handleKeyDown}
         />
      </header>
    </div>
  );
}

export default OpenChat;