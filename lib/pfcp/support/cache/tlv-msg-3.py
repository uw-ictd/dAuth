ies = []
ies.append({ "ie_type" : "Application ID's PFDs", "ie_value" : "Application ID's PFDs", "presence" : "C", "tlv_more" : "0", "comment" : "This IE shall contain an Application Identifier and the associated PFDs to be provisioned in the UP function.Several IEs with the same IE type may be present to provision PFDs for multiple Application IDs.The UP function shall delete all the PFDs received and stored earlier for all the Application IDs if this IE is absent in the message."})
msg_list[key]["ies"] = ies
