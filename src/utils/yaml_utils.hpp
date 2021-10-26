//
// This file is a part of UERANSIM open source project.
// Copyright (c) 2021 ALİ GÜNGÖR.
//
// The software and all associated files are licensed under GPL-3.0
// and subject to the terms and conditions defined in LICENSE file.
//

#pragma once

#include <optional>
#include <string>
#include <vector>

namespace YAML
{
class Node;
}

namespace yaml
{

bool HasField(const YAML::Node &node, const std::string &name);
void AssertHasField(const YAML::Node &node, const std::string &name);
void AssertHasFields(const YAML::Node &node, const std::vector<std::string> &fields);

void AssertHasInt32(const YAML::Node &node, const std::string &name);
void AssertHasInt64(const YAML::Node &node, const std::string &name);
void AssertHasString(const YAML::Node &node, const std::string &name);
void AssertHasBool(const YAML::Node &node, const std::string &name);
void AssertHasSequence(const YAML::Node &node, const std::string &name);

int GetInt32(const YAML::Node &node, const std::string &name);
int GetInt32(const YAML::Node &node, const std::string &name, std::optional<int> minValue, std::optional<int> maxValue);

int64_t GetInt64(const YAML::Node &node, const std::string &name);
int64_t GetInt64(const YAML::Node &node, const std::string &name, std::optional<int64_t> minValue,
                 std::optional<int64_t> maxValue);

std::string GetString(const YAML::Node &node, const std::string &name);
std::string GetString(const YAML::Node &node, const std::string &name, std::optional<int> minLength,
                      std::optional<int> maxLength);

std::string GetIp4(const YAML::Node &node, const std::string &name);

bool GetBool(const YAML::Node &node, const std::string &name);

std::vector<YAML::Node> GetSequence(const YAML::Node &node, const std::string &name);

} // namespace yaml
