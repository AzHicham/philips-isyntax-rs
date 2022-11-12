#include "facade.hpp"

void open(Facade& facade, std::string const& url, std::string const& containerName, std::string const& cacheName){
   facade.open(url, containerName, std::ios::in | std::ios::binary, cacheName);
}

SubImage const& sub_image(const Facade& facade, std::string const& image_type) {
    return facade[image_type];
}
