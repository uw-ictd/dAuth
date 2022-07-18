import json
from statistics import mean
from copy import deepcopy
from typing import Dict, Set, List


class PerfMetrics:
    """
    Maintains the set of performance metrics for a given perf test.
    """
    
    def __init__(self, test_name: str) -> None:
        self._results: Dict[str, Dict[str, List[int]]] = dict()
        self._tags: Dict[str, List[int]] = dict()
        self.test_name = test_name
        self.test_time = 0.0
        self.total_auths = 0
        
    def get_names(self) -> Set[str]:
        """
        Returns the set of all of the names/imsis in this test.
        """
        return set(self._results.keys())
    
    def get_command_tags(self, name: str) -> List[int]:
        """
        Returns the set of command tags for a given name.
        """
        return deepcopy(self._tags.get(name))
    
    def get_results(self, name: str) -> Dict[str, List[int]]:
        """
        Returns a mapping of measurement type to set of measurements for 
        a given name.
        """
        return deepcopy(self._results.get(name))
    
    def get_average(self, name: str) -> Dict[str, int]:
        """
        Returns a new mapping of result type to average results for a 
        given name.
        """
        results = self.get_results(name)
        averages = dict()
        
        for res_type, res_list in results.items():
            averages[res_type] = mean(res_list)
        
        return averages
    
    def get_total_average(self) ->  Dict[str, int]:
        """
        Returns a new mapping of result type to average results for all names.
        """
        sums: Dict[str, List[int]] = dict()
        for name in self.get_names():
            results = self.get_results(name)
            
            if len(sums) == 0:
                for res_type, res_list in results.items():
                    sums[res_type] = res_list
            else:
                for res_type, res_list in results.items():
                    sums[res_type].extend(res_list)
  
        averages = dict()
        
        for res_type, res_list in sums.items():
            averages[res_type] = mean(res_list)
        
        return averages
    
    def get_results_json(self) -> str:
        """
        Gets all results outputed in a single json string.
        """
        res = dict()
        res["test_name"] = self.test_name
        res["test_duration"] = self.test_time
        res["total_auths"] = self.total_auths
        res["nanoseconds_since_auth"] = list()
        res["nanoseconds_since_registration"] = list()
        for name in self.get_names():
            test_res = self.get_results(name)
            res["nanoseconds_since_auth"].extend(test_res["nanoseconds_since_auth"])
            res["nanoseconds_since_registration"].extend(test_res["nanoseconds_since_registration"])
            # res[name] = {
            #     "results": self.get_results(name),
            #     "averages": self.get_average(name),
            # }
        # res["total_averages"] = self.get_total_average()

        res["total_auths"] = len(res["nanoseconds_since_auth"])
        
        return json.dumps(res)
    
    def add_result_from_json(self, json_string: str) -> None:
        """
        Attempts to parse a json from the provided string and store the results.
        Throws an exception on incorrect json and incorrect data.
        
        Expects the following structure:
            '{
                "nanoseconds_since_auth":<int>,
                "nanoseconds_since_registration":<int>,
                "ue_supi":"91054xxxxxxxxxx",
             }'
        """
        dataset = json.loads(json_string)
        
        name = dataset["ue_supi"]
        
        if name not in self._results:
            self._results[name] = {
                "nanoseconds_since_auth":
                    [dataset["nanoseconds_since_auth"]],
                "nanoseconds_since_registration":
                    [dataset["nanoseconds_since_registration"]],
            }
        else:
            results: Dict[str, List[int]] = self._results[name]
            results["nanoseconds_since_auth"].append(
                dataset["nanoseconds_since_auth"])
            results["nanoseconds_since_registration"].append(
                dataset["nanoseconds_since_registration"])
