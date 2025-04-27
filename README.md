# capbot

Discord bot (capbot)

-   Use prefix `.`
-   Current commands include `.help` (does nothing yet) and `.view` (can access the NHL API).
-   To send post requests to server upon starting up, check your machine's IP and send to `http://<IP ADDR>/capbot`.
    ```bash
    e.g. curl -X POST -H "Content-Type: text/plain" --data "Hello capbot" http://<IP ADDR>/capbot
    ```
-   Environment variables that should be in your `.env` file:
    ```dotenv
    # copy from your developer portal
    DISCORD_TOKEN=YOUR_DISCORD_TOKEN

    # copy from your channel
    BAD_DATA_CHANNEL_ID=YOUR_CHANNEL_ID

    # change port if needed
    HTTP_LISTEN_ADDR=0.0.0.0:3000
    ```
    