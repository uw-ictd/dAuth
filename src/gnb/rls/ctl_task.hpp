//
// This file is a part of UERANSIM open source project.
// Copyright (c) 2021 ALİ GÜNGÖR.
//
// The software and all associated files are licensed under GPL-3.0
// and subject to the terms and conditions defined in LICENSE file.
//

#pragma once

#include "udp_task.hpp"

#include <gnb/nts.hpp>
#include <gnb/types.hpp>
#include <utils/nts.hpp>

namespace nr::gnb
{

class RlsControlTask : public NtsTask
{
  private:
    std::unique_ptr<Logger> m_logger;
    uint64_t m_sti;
    NtsTask *m_mainTask;
    RlsUdpTask *m_udpTask;
    std::unordered_map<uint32_t, rls::PduInfo> m_pduMap;
    std::unordered_map<int, std::vector<uint32_t>> m_pendingAck;

  public:
    explicit RlsControlTask(TaskBase *base, uint64_t sti);
    ~RlsControlTask() override = default;

  protected:
    void onStart() override;
    void onLoop() override;
    void onQuit() override;

  public:
    void initialize(NtsTask *mainTask, RlsUdpTask *udpTask);

  private:
    void handleSignalDetected(int ueId);
    void handleSignalLost(int ueId);
    void handleRlsMessage(int ueId, rls::RlsMessage &msg);
    void handleDownlinkRrcDelivery(int ueId, uint32_t pduId, rrc::RrcChannel channel, OctetString &&data);
    void handleDownlinkDataDelivery(int ueId, int psi, OctetString &&data);
    void onAckControlTimerExpired();
    void onAckSendTimerExpired();
};

} // namespace nr::gnb