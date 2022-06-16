import json
from statistics import mean
from copy import deepcopy
from typing import Dict, Set, List


class PerfMetrics:
    """
    Maintains the set of performance metrics for a given perf test.
    """
    
    def __init__(self) -> None:
        self._results: Dict[str, Dict[str, List[int]]] = dict()
        self._tags: Dict[str, List[int]] = dict()
        
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
    
    def add_result_from_json(self, json_string: str) -> None:
        """
        Attempts to parse a json from the provided string and store the results.
        Throws an exception on incorrect json and incorrect data.
        
        Expects the following structure:
            '{
                "result":"Ok",
                "nanoseconds_since_auth":<int>,
                "nanoseconds_since_registration":<int>,
                "nanoseconds_to_establish_session":<int>,
                "ue_supi":"90170xxxxxxxxxx",
                "command_tag":<int>
             }'
        """
        dataset = json.loads(json_string)
        
        if dataset["result"] != "Ok":
            raise Exception("Invalid result state: {}".format(dataset["result"]))
        
        name = dataset["ue_supi"]
        
        if name not in self._results:
            self._results[name] = {
                "nanoseconds_since_auth":
                    [dataset["nanoseconds_since_auth"]],
                "nanoseconds_since_registration":
                    [dataset["nanoseconds_since_registration"]],
                "nanoseconds_to_establish_session":
                    [dataset["nanoseconds_to_establish_session"]],
            }
        else:
            results: Dict[str, List[int]] = self._results[name]
            results["nanoseconds_since_auth"].append(
                dataset["nanoseconds_since_auth"])
            results["nanoseconds_since_registration"].append(
                dataset["nanoseconds_since_registration"])
            results["nanoseconds_to_establish_session"].append(
                dataset["nanoseconds_to_establish_session"])
        
        if name not in self._tags:
            self._tags[name] = list()
        
        self._tags[name].append(dataset["command_tag"])
