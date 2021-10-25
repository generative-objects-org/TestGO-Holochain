
import { Orchestrator } from "@holochain/tryorama";

import my_test_go_zome from './myTestGO-DNA/my_test_go_zome';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
my_test_go_zome(orchestrator);
orchestrator.run();



