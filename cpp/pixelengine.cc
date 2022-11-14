#include "pixelengine.hpp"
#include "PhilipsPixelEngine/eglrendercontext.hpp"
#include "PhilipsPixelEngine/gles2renderbackend.hpp"
#include "PhilipsPixelEngine/softwarerenderbackend.hpp"
#include "PhilipsPixelEngine/softwarerendercontext.hpp"

std::unique_ptr<PixelEngine> make_pixel_engine(const std::unique_ptr<RenderContext>&,
                                               const std::unique_ptr<RenderBackend>&);

std::unique_ptr<PixelEngine> make_pixel_engine(const std::unique_ptr<RenderContext>& render_context,
                                               const std::unique_ptr<RenderBackend>& render_backend) {
    return std::make_unique<PixelEngine>(*render_backend, *render_context);
}

std::unique_ptr<RenderContext> make_render_context() { return std::make_unique<SoftwareRenderContext>(); }

std::unique_ptr<RenderBackend> make_render_backend() { return std::make_unique<SoftwareRenderBackend>(); }

std::unique_ptr<std::string> pe_version() { return std::make_unique<std::string>(PixelEngine::version()); }

Facade& facade(PixelEngine& pixel_engine, std::string const& name) { return pixel_engine[name]; }

/*
void waitAll(PixelEngine& pixel_engine, std::vector<SharedPtrRegion> const& regions) {
    std::list<std::shared_ptr<Region>> _regions(regions.begin(), regions.end());
    pixel_engine.waitAll(_regions);
}*/
