#! /usr/bin/env bash
# x is for execute

function deps() {
  echo ""
  read -p "This will install rust, sqlx, and docker-compose if they are absent. It will check for but not install docker. Continue? (Y or n) " -n 1 -r
  echo ""
  if [[ ! $REPLY =~ ^[Yy]$ ]]
  then
    exit 1
  fi

  if command -v cargo >/dev/null 2>&1; then
    :
  else
    echo "installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    if [ -f $HOME/.cargo/env ]; then
      source $HOME/.cargo/env
    fi
  fi

  if command -v sqlx >/dev/null 2>&1; then
    :
  else
    echo "installing sqlx"
    cargo install sqlx-cli
  fi

  if command -v docker-compose >/dev/null 2>&1; then
    :
  else
    echo "docker not installed. See https://docs.docker.com/engine/install/ for instructions installing."
  fi

  if command -v docker-compose >/dev/null 2>&1; then
    :
  else
    sudo curl -L "https://github.com/docker/compose/releases/download/1.27.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
  fi
}

function x() {
  local cmd=$1
  local serve_asset=lastresort-server
  local seed_asset=lastresort-seeder
  local cmds="buildfed buildseed buildserve checkseed checkserve dbadd dbcli dbcreate dbmigrate dbreset dbup deps doc mkseed mkserve seed serve"
  local tgt_dir="./target/x86_64-unknown-linux-musl/debug"
  local db_name="$LASTRESORT_DB"
  local db_user="$LASTRESORT_PGUSER"
  local postgres_version="13"
  case $cmd in
    buildfed)
      pushd client
      yarn && yarn build
      popd
      ;;
    buildseed)
      cargo build --color=never --release --bin $seed_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    buildserve)
      cargo build --color=never --release --bin $serve_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    checkseed)
      cargo check --bin $seed_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    checkserve)
      cargo check --bin $serve_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    dbadd)
      sqlx migrate add "${@:2}"
      ;;
    dbcli)
      if command -v psql >/dev/null 2>&1; then
        psql -h 127.0.0.1 --password "$db_name" "$db_user"
      else
        echo "install psql, then rerun this command"
        echo "on Debian, that could look like:"
        echo "sudo apt update && sudo apt install postgresql-client-common postgresql-client-$postgres_version"
      fi
      ;;
    dbcreate)
      sqlx database create
      ;;
    dbmigrate)
      sqlx migrate run
      ;;
    dbreset)
      sqlx database reset
      ;;
    dbup)
      docker-compose -f compose-db.yml up
      ;;
    deps)
      deps
      ;;
    doc)
      cargo doc --features vendored "${@:2}"
      ;;
    mkseed)
      cargo build --color=never --bin $seed_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    mkserve)
      cargo build --color=never --bin $serve_asset --features vendored --target x86_64-unknown-linux-musl
      ;;
    seed)
      if [ -f $tgt_dir/$seed_asset ]; then
        $tgt_dir/$seed_asset
      else
        echo "build the seeder with `./x mkseed`, then rerun this command"
      fi
      ;;
    serve)
      if [ -f $tgt_dir/$serve_asset ]; then
        $tgt_dir/$serve_asset
      else
        echo "build the server with `./x mkserve`, then rerun this command"
      fi
      ;;
    *)
      echo "Can't \"$1\". Try one of: $cmds"
      ;;
  esac
}

x "$@"
