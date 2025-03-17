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

    this->config_directory.path = GetConfigDirectory();
    this->config_directory.Create();
}

const char* Core::GetConfigDirectory()
{
    #ifdef _WIN32
        const char* homeDir = std::getenv("USERPROFILE");
        if (homeDir) {
            static std::string configPath = std::string(homeDir) + "\\.harbour";
            return configPath.c_str();
        }
    #else
        const char* homeDir = std::getenv("HOME");
        if (homeDir) {
            static std::string configPath = std::string(homeDir) + "/.harbour";
            return configPath.c_str();
        }
    #endif
}

void Core::Parse()
{
    spdlog::info("Welcome");

    this->arguments.Parse(this->argc, this->argv);
}
