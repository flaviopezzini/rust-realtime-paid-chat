use STOMP on websocket.

Implement websockets over TLS
OWASP validator on messages.
Only allow 2 users in a chat, one has to be advisor, other a regular customer.
Only allow who's not busy to join a chat.

https://payatu.com/blog/websocketsecurity/
useWebSocket is probably using useEffect to add callbacks to the internal message listeners, so if your component re-renders your callbacks will be different and the internal useEffect is probably resetting the connection because of that.
moving to a separate component probably fixed it because it's no longer re-rendering as often but the problem might still me there. with libraries like these try to memoize your callbacks with useCallback and the opts object with useMemo to avoid things like that
