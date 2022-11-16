.PHONY: install-deps dl-test-images

install-deps:
	export DEBIAN_FRONTEND=noninteractive && \
	sudo apt-get update -qq && \
	sudo apt-get install -y --no-install-recommends unzip binutils build-essential && \
	gsutil cp gs://az-philips/sdk.zip . && \
	unzip sdk.zip -d . && \
	cd sdk && \
	chmod +x InstallPathologySDK.sh && \
    sudo ./InstallPathologySDK.sh -y

dl-test-images:
	gsutil cp gs://az-philips/sample.isyntax tests/data/
