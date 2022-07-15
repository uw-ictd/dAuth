for p in {0..9}
do
    for u in {0..9}
    do
        for l in {0..9}
        do
            if [ "000" == "$p$u$l" ]
            then
                echo adding "imsi-901700000001000"
                sudo ./open5gs-dbctl add "901700000001000" "465B5CE8B199B49FAA5F0A2EE238A6BC" "E8ED289DEBA952E4283B54E88E6183CA"
            else
                echo adding "imsi-901700000000$p$u$l"
                sudo ./open5gs-dbctl add "901700000000$p$u$l" "465B5CE8B199B49FAA5F0A2EE238A6BC" "E8ED289DEBA952E4283B54E88E6183CA"
            fi
        done
    done
done
