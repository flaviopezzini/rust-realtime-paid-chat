import './OpenChat.css';

import React, { useState, KeyboardEvent } from 'react';

import useWebSocket from 'react-use-websocket';

type Props = {
  username: string,
  userType: String,
}

function OpenChat(props: Props) {

  const [messageHistory, setMessageHistory] = useState('');
  const [newMessageText, setNewMessageText] = useState('');

  const socketUrl = 'ws://localhost:9000/websocket';

  const {
    sendMessage,
//    sendJsonMessage,
//     lastMessage,
//     lastJsonMessage,
//     readyState,
//     getWebSocket,
  } = useWebSocket(socketUrl, {
    onOpen: () => {
      sendMessage(JSON.stringify(props));
    },
    onClose: () => {
        console.log("closed");
    },
    onMessage: (e) => {
      setMessageHistory((prev) => prev.concat(e.data+"\r\n"));
    },
    //Will attempt to reconnect on all close events, such as server shutting down
    shouldReconnect: () => true,
  });

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNewMessageText(event.target.value);
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      sendMessage(newMessageText);
      setNewMessageText(() => "");
    }
  };

  return (
    <div className="OpenChat">
      <header className="OpenChat-header">
        <textarea id="chat" className="chat-area" cols={30} rows={10}
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