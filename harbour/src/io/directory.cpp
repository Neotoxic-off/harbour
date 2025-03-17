#include "io/directory.hpp"

Directory::Directory()
{
}

Directory::~Directory()
{
}

bool Directory::Exists()
{
    this->exists = std::filesystem::exists(path) && std::filesystem::is_directory(path);

    return this->exists;
}

std::vector<std::string> Directory::List()
{
    std::vector<std::string> contents;

    for (const auto& entry : std::filesystem::directory_iterator(path)) {
        contents.push_back(entry.path().string());
    }

    return contents;
}

bool Directory::Create()
{
    return std::filesystem::create_directories(this->path);
}

bool Directory::Delete()
{
    return std::filesystem::remove_all(path) > 0;
}
