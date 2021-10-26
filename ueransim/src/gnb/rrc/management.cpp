//
// This file is a part of UERANSIM open source project.
// Copyright (c) 2021 ALİ GÜNGÖR.
//
// The software and all associated files are licensed under GPL-3.0
// and subject to the terms and conditions defined in LICENSE file.
//

#include "task.hpp"

namespace nr::gnb
{

RrcUeContext *GnbRrcTask::tryFindByInitialRandomId(int64_t id)
{
    if (id == -1)
        return nullptr;
    // TODO: Optimize
    for (auto &item : m_ueCtx)
        if (item.second->initialRandomId == id)
            return item.second;
    return nullptr;
}

int GnbRrcTask::getNextTid()
{
    m_tidCounter++;
    m_tidCounter %= 4;
    return m_tidCounter;
}

} // namespace nr::gnb
