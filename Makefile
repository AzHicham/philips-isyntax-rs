.PHONY: install-deps

install-deps-apt:
	export DEBIAN_FRONTEND=noninteractive && \
	sudo apt-get update -qq && \
	sudo apt-get install -y --no-install-recommends unzip binutils build-essential && \
	gsutil cp gs://az-philips/sdk.zip . && \
	unzip sdk.zip -d . && \
	cd sdk && \
	chmod +x InstallPathologySDK.sh && \
    sudo ./InstallPathologySDK.sh -y
