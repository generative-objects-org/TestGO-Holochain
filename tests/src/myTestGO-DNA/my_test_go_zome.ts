
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { last } from "lodash";
import { config, installation, sleep } from '../utils';

export default (orchestrator: Orchestrator<any>) => 
  orchestrator.registerScenario("my_test_go_zome tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellNick.includes('/myTestGO-DNA.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellNick.includes('/myTestGO-DNA.dna')) as Cell;


    // Test creation of a first Customer
    const first_name1 = "Charles";
    const last_name1 = "Almeida";
    
    const customerHash1 = await alice.call(
      "my_test_go_zome",
      "create_customer",
      { first_name : first_name1, last_name : last_name1 }
    );
    
    console.log("alice created the customer : ", customerHash1);
    t.ok(customerHash1);

    // Test creation of a second Customer
    const first_name2 = "Walter";
    const last_name2 = "Almeida";
    
    const customerHash2 = await alice.call(
      "my_test_go_zome",
      "create_customer",
      { first_name : first_name2, last_name : last_name2 }
    );
    
    console.log("alice created the customer : ", customerHash2);
    t.ok(customerHash2);
    
    await sleep(50);
    
    // Test get first created customer, from another user
    const customer = await bob.call(
      "my_test_go_zome",
      "get_customer",
      customerHash1
    );

    console.log("bob retrieved the customer : ", customerHash1, customer.first_name, " ", customer.last_name);

    t.ok(customer.first_name == first_name1 && customer.last_name == last_name1);

    // Test get all customers
    const all_customers = await bob.call(
      "my_test_go_zome",
      "get_all_customers"
    );
    
    console.log("Retrieved all customers : ", all_customers);

    t.ok(all_customers);
    
});
