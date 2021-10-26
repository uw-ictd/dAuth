ies = []
ies.append({ "iei" : "28", "value" : "5GSM capability", "type" : "5GSM capability", "reference" : "9.11.4.1", "presence" : "O", "format" : "TLV", "length" : "3-15"})
ies.append({ "iei" : "59", "value" : "5GSM cause", "type" : "5GSM cause", "reference" : "9.11.4.2", "presence" : "O", "format" : "TV", "length" : "2"})
ies.append({ "iei" : "55", "value" : "Maximum number of supported packet filters", "type" : "Maximum number of supported packet filters", "reference" : "9.11.4.9", "presence" : "O", "format" : "TV", "length" : "3"})
ies.append({ "iei" : "B-", "value" : "Always-on PDU session requested", "type" : "Always-on PDU session requested", "reference" : "9.11.4.4", "presence" : "O", "format" : "TV", "length" : "1"})
ies.append({ "iei" : "13", "value" : "Integrity protection maximum data rate", "type" : "Integrity protection maximum data rate", "reference" : "9.11.4.7", "presence" : "O", "format" : "TV", "length" : "3"})
ies.append({ "iei" : "7A", "value" : "Requested QoS rules", "type" : "QoS rules", "reference" : "9.11.4.13", "presence" : "O", "format" : "TLV-E", "length" : "7-65538"})
ies.append({ "iei" : "79", "value" : "Requested QoS flow descriptions", "type" : "QoS flow descriptions", "reference" : "9.11.4.12", "presence" : "O", "format" : "TLV-E", "length" : "6-65538"})
ies.append({ "iei" : "75", "value" : "Mapped EPS bearer contexts", "type" : "Mapped EPS bearer contexts", "reference" : "9.11.4.8", "presence" : "O", "format" : "TLV-E", "length" : "7-65538"})
ies.append({ "iei" : "7B", "value" : "Extended protocol configuration options", "type" : "Extended protocol configuration options", "reference" : "9.11.4.6", "presence" : "O", "format" : "TLV-E", "length" : "4-65538"})
ies.append({ "iei" : "7C", "value" : "Port management information container", "type" : "Port management information container", "reference" : "9.11.4.27", "presence" : "O", "format" : "TLV-E", "length" : "3-65538"})
ies.append({ "iei" : "66", "value" : "Header compression configuration", "type" : "Header compression configuration", "reference" : "9.11.4.24", "presence" : "O", "format" : "TLV", "length" : "5-257"})
msg_list[key]["ies"] = ies
