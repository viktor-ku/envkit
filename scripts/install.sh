P_OSX=OSX
P_LINUX=LINUX

ref=0.2.0
dest=/usr/local/bin

function call_download {
  local path=$1
  local job=$2

  echo Downloading envkit $ref...
  curl -o envkit -L "https://gitlab.com/viktor-ku/envkit/-/jobs/artifacts/$ref/raw/$path?job=$job"
}

function call_download_linux {
  local job=build-linux
  local path=target/release/envkit
  call_download $path $job
}

function call_download_osx {
  local job=build-osx
  local path=target/x86_64-apple-darwin/release/envkit
  call_download $path $job
}

function call_message_installed {
  echo $(envkit --version) has been installed! Run \"envkit --help\" to get you started
}

function call_install {
  local current_version=$(envkit --version 2>> /dev/null)
  if [[ $current_version == "envkit $ref" ]]; then
    echo $current_version
    echo You have the latest version! Run \"envkit --help\" to get you started
    exit 0
  fi

  cd /tmp

  local downloaded_version=$(./envkit --version 2>> /dev/null)
  if [[ $downloaded_version == "envkit $ref" ]]; then
    echo Installing $ref...
    sudo mkdir -p $dest
    sudo cp -f ./envkit $dest/envkit
    call_message_installed
    exit 0
  fi

  local platform=$1
  if [[ $platform == $P_LINUX ]]; then
    call_download_linux
  elif [[ $platform == $P_OSX ]]; then
    call_download_osx
  fi

  echo Installing $ref...
  chmod +x ./envkit
  sudo mkdir -p $dest
  sudo cp -f ./envkit $dest/envkit
  call_message_installed
}

function call_main {
  if [[ $OSTYPE == linux-gnu ]]; then
    call_install $P_LINUX
  elif [[ $OSTYPE == darwin* ]]; then
    call_install $P_OSX
  else
    echo Current platform \"$OSTYPE\" is not supported at the moment.
    echo Open the issue here https://gitlab.com/viktor-ku/envkit/issues
    exit 1
  fi
}

call_main
