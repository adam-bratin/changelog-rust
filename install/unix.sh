fetch_file()
{
	local url=$1
	local target=$2

	if [ -x "/usr/bin/wget" ]; then
		cmd="/usr/bin/wget $url -O $target"
	elif [ -x "/usr/bin/curl" ]; then
		cmd="/usr/bin/curl -L $url -o $target"
	else
		die "Unable to find download manager(wget, curl)"
	fi

	echo "Transport command is $cmd"

	$cmd
}

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)
	machine=Linux
	zip=x86_64-unknown-linux-musl.zip
	;;
    Darwin*)
	machine=Mac
	zip=x86_64-apple-darwin.zip
	;;
    *)          machine=UNKNOWN
esac
if [ "$unameOut" = "UNKNOWN" ]; then
    echo "Cannot run script on unsupported OS"
    exit 1
fi
TEMP=$(mktemp /tmp/$zip.XXXXXXXXXX)
fetch_file "https://github.com/adam-bratin/changelog-rust/releases/latest/download/$zip" ${TEMP}
if [ ! -x "$(which unzip)" ]; then
	while true; do
		read -p "Do you wish to install unzip? Y/N" yn
		case $yn in
			[Yy]* ) break;;
			[Nn]* ) exit 1;;
			* ) echo "Please answer yes or no.";;
		esac
	done
	if ["$machine" -eq "Linux"]; then
		apt install unzip
	fi
fi

unzip ${TEMP} -d /usr/local/bin
chmod +x /usr/local/bin/changelog-rust
