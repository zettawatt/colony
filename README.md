<p align="center">
 <img align="center" src="https://raw.githubusercontent.com/zettawatt/colony/main/static/logo-192x192.png" height="96" />
 <h1 align="center">
  colony
 </h1>
</p>

colony is a static site written in Svelte with the eventual goal of it being uploaded to the Autonomi network and also with the option to be self-hosted on a personal webserver. So far, it is a skeleton Svelte site and does nothing interesting, but it will soon!

The colony application can be viewed on the web here: https://colonyapp.org/
This site will be updated as features are added.

## Setup

Install NodeJS. Follow instructions for your OS. For Ubuntu variants:

```bash
curl -fsSL https://deb.nodesource.com/setup_23.x -o nodesource_setup.sh
sudo -E bash nodesource_setup.sh
sudo apt-get install -y nodejs
```

Install dependencies:

```bash
cd colony
npm install
```

## Developing

Once you've cloned the repo, start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To build a static version of this site:

```bash
npm run build
```

You can preview the production build with `npm run preview`.
