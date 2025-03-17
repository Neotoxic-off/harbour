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

    if (this->config_directory.Exists() == false) {
        this->config_directory.Create();
    }
}

std::string Core::GetConfigDirectory()
{
    #ifdef _WIN32
        const char* homeDir = std::getenv("USERPROFILE");
    #else
        const char* homeDir = std::getenv("HOME");
    #endif

    if (homeDir) {
        std::filesystem::path configPath = std::filesystem::path(homeDir) / CONFIG_DIRECTORY;

        return configPath.string();
    }

    return CONFIG_DIRECTORY;
}

void Core::Parse()
{
    spdlog::info("Welcome");

    this->arguments.Parse(this->argc, this->argv);
}
