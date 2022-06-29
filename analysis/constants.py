def label_from_scenario(scenario):
    scenario = int(scenario)
    if scenario == 1:
        return "1- Edge PC Industrial Fiber"
    elif scenario == 2:
        return "2- Edge PC Residential Cable"
    elif scenario == 3:
        return "3- Cloud Host Industrial Fiber"
    elif scenario == 4:
        return "4- Cloud Host Residential Cable"
    else:
        raise ValueError("Unknown scenario")
