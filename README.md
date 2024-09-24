# How to use


### 1 - Download and extract the latest release
To get started download the [Streamrate_v1.0.zip](https://github.com/Streamrate/Streamrate/releases/download/release/streamrate_v1.0.zip) and then extract to an accessible directory

### 2 - Add your obs websocket password in  the .env file

open obs **OBS > Tools > Websocket server settings** and check the **Enable WebSocket server** and **Enable Authorization** if not enabled and click on the **Apply** button. then click on **Show Connect Info** button and copy the **Server Password**.

open  the **.env** file in notepad (or any other text editor) and enter your password. your **.env file** should look like this: (in the following example my obs websocket password is **m3vVwU4SkJlfC8au**
```.env
OBS_WEBSOCKET_PASSWORD=m3vVwU4SkJlfC8au
```
save the env file and simply click on the **run.bat** file.

**NOTE**: Currently there is no GUI so you have to run the **run.bat** file each time you stream. GUI with more features will be added soon.

## 3 Show bitrate data in OBS

Open the OBS and in the **sources** secition click on the **+** button and choost **Text (GDI+)** the click on **create new** and check the **Read from file** option and choose the generated **obs_bitrate.txt** file.

your bitrate should be visible while streaming, otherwise it will show an "Stream not active" text
