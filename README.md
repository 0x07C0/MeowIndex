# MeowIndex

A cute, feature-rich file listing module to replace nginx's autoindex / fancyindex.


## Features

* [x] List files
* [x] Show file icons

**TODO**

* [ ] Infinite-scroll
* [ ] Search
* [ ] Show image/video previews
* [ ] Show 404 page


## How to use

### 1. Installation

You can install from source by cloning the repository:

```sh
cd /etc/nginx
git clone https://github.com/hykilpikonna/MeowIndex
cd MeowIndex
yarn install
yarn build
```

To update, simply do `git pull` and `yarn build` in the same directory.

### 2. Setup File Listing in Nginx

This module uses the json file listing api in nginx. If you already have an autoindex file server set up, you can make the following changes. If you're new to nginx, you can check out our [example configs](docs/examples).

The following example serves `/data/file-server` on http path `/`

```diff
server_name your.domain.com;

- root /data/file-server;
+ set $dir_path /data/file-server;
+ include "/etc/nginx/MeowIndex/docs/nginx.conf";

- location / {
-     fancyindex on;
-     fancyindex_exact_size off;
- }

+ location / {
+     try_files $uri $uri/index.html /__meowindex__/index.html;
+ }
```