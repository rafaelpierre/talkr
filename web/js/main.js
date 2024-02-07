document.addEventListener('DOMContentLoaded', function() {
    const sendMessageButton = document.getElementById('send-message');
    const messageInputField = document.getElementById('chat-input-field');
    const chatMessagesContainer = document.getElementById('chat-messages');

    sendMessageButton.addEventListener('click', function() {
        sendMessage();
    });

    messageInputField.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            sendMessage();
            e.preventDefault(); // Prevent the default action to stop form submission
        }
    });

    function sendMessage() {
        const messageText = messageInputField.value.trim();
        if (messageText) {
            // Create a container for the message and the avatar
            const messageContainer = document.createElement('div');
            messageContainer.classList.add('message-container', 'user-message-container');

            // Create the avatar element
            const avatarImg = document.createElement('img');
            avatarImg.src = 'img/user.webp'; // Path to your user avatar image
            avatarImg.alt = 'User Avatar';
            avatarImg.classList.add('avatar');

            // Create the message bubble
            const newMessageDiv = document.createElement('div');
            newMessageDiv.classList.add('message', 'user-message');
            newMessageDiv.innerHTML = `<p>${messageText}</p>`;

            // Append the avatar and message bubble to the message container
            messageContainer.appendChild(avatarImg);
            messageContainer.appendChild(newMessageDiv);

            // Append the message container to the chat messages container
            chatMessagesContainer.appendChild(messageContainer);
            messageInputField.value = ''; // Clear input field after sending message

            // Scroll to the bottom of the chat messages container
            chatMessagesContainer.scrollTop = chatMessagesContainer.scrollHeight;
        }
    }
});
