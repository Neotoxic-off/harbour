#include "io/file.hpp"

File::File()
{
}

File::~File()
{

}

bool File::Exists()
{
    this->exists = std::filesystem::exists(this->path);

    return this->exists;
}

void File::Write(const std::string content)
{
    std::ofstream file(this->path);
    if (file.is_open()) {
        file << content;
        file.close();
    }
}

std::string File::Read()
{
    std::string content;

    std::ifstream file(this->path);
    if (file.is_open()) {
        std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
        file.close();
    }

    return content;
}

bool File::Create()
{
    bool status = false;
    std::ofstream output(this->path);

    status = output.is_open();
    output.close();

    return status;
}

bool File::Delete()
{
    return std::filesystem::remove(this->path);
}
