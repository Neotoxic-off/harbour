#include "core.hpp"

Core::Core(int argc, char** argv)
{
    this->argc = argc;
    this->argv = argv;
}

Core::~Core()
{
    
}

void Core::Initialize()
{
    spdlog::set_level(spdlog::level::debug);
}

void Core::Parse()
{
    spdlog::info("Welcome");

    this->arguments.Parse(this->argc, this->argv);
}
