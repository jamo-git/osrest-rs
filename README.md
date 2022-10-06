# A simple REST API made in rust

This simple REST application that will return OS name & version, uptime in JSON. It's made for Windows & Linux. The application will run on port `:8000` and endpoints are routed:
- `:8000/` will inform other existing endpoints
- `:8000/sysinfo` will return os_name and os_version in JSON
- `:8000/uptime` will return os_name and os_version in JSON