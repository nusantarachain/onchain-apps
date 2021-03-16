<template>
  <div class="container">
    <div>
      <h1 class="title">UMKM4</h1>

      <div class="grid grid-cols-2 gap-4 grid-flow-row-dense">

          <div
            v-if="errorInfo != ''"
            class="col-span-2 flex items-center bg-red-200 text-red-600 text-sm font-bold mx-5 px-10 py-3 mb-5"
            role="alert"
          >
            <svg
              class="w-4 h-4 mr-2"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
            >
              <path
                d="M12.432 0c1.34 0 2.01.912 2.01 1.957 0 1.305-1.164 2.512-2.679 2.512-1.269 0-2.009-.75-1.974-1.99C9.789 1.436 10.67 0 12.432 0zM8.309 20c-1.058 0-1.833-.652-1.093-3.524l1.214-5.092c.211-.814.246-1.141 0-1.141-.317 0-1.689.562-2.502 1.117l-.528-.88c2.572-2.186 5.531-3.467 6.801-3.467 1.057 0 1.233 1.273.705 3.23l-1.391 5.352c-.246.945-.141 1.271.106 1.271.317 0 1.357-.392 2.379-1.207l.6.814C12.098 19.02 9.365 20 8.309 20z"
              />
            </svg>
            <p>{{ errorInfo }}</p>
          </div>


        <div
          class="shadow-xl p-10 bg-white rounded row-span-1 col-span-2 box-content h-20"
        >
          <div class="grid grid-cols-4 gap-4">
            <div class="flex p-5 col-span-2">
              <span
                class="text-sm border border-2 rounded-1 px-4 py-2 bg-gray-300 whitespace-no-wrap"
                >Contract Address</span
              >
              <input
                type="text"
                class="border border-2 rounded-r px-4 py-2 w-full"
                name="ContractAddress"
                placeholder="eg: 5F8BBh3SG9EYxWQeWadksPFMfDSzchaXvUafB4KGnkntpHC7"
                v-model="contractAddress"
                :disabled="connecting"
              />
              <button
                class="bg-green-600 hover:bg-green-500 text-white font-bold py-3 px-6 rounded"
                @click="reconnect()"
                :disabled="connecting"
              >
                Connect
              </button>
            </div>
            <div>
              <h5 class="text-gray-500 uppercase font-bold text-xs">Status</h5>
              <span class="text-gray-800" v-if="connecting">connecting...</span>
              <span class="font-bold text-xl text-gray-800" v-if="!connecting">
                {{ connected ? "connected" : "not connected" }}
              </span>
            </div>
            <div>
              <h5 class="text-gray-500 uppercase font-bold text-xs">
                Total Members
              </h5>
              <span class="font-semibold text-xl text-gray-800">
                {{ memberCount }}
              </span>
            </div>
          </div>
        </div>
        <div
          class="shadow-xl p-10 bg-white rounded row-span-1 col-span-2 box-content h-20"
        >
          <div v-if="selectedAccount">
            <label
              id="listbox-label"
              class="block text-sm font-medium text-gray-700"
            >
              Use Account: (Injected from extension)
            </label>
            <div class="mt-1 relative">
              <button
                type="button"
                aria-haspopup="listbox"
                aria-expanded="true"
                aria-labelledby="listbox-label"
                class="relative w-full bg-white border border-gray-300 rounded-md shadow-sm pl-3 pr-10 py-2 text-left cursor-default focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                @click="expandAccounts = !expandAccounts"
              >
                <span class="flex items-center">
                  <span class="ml-3 block truncate" v-if="selectedAccount.meta">
                    <strong>{{ selectedAccount.meta.name }}</strong> |
                    {{ selectedAccount.address }}
                  </span>
                </span>
                <span
                  class="ml-3 absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none"
                >
                  <svg
                    class="h-5 w-5 text-gray-400"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M10 3a1 1 0 01.707.293l3 3a1 1 0 01-1.414 1.414L10 5.414 7.707 7.707a1 1 0 01-1.414-1.414l3-3A1 1 0 0110 3zm-3.707 9.293a1 1 0 011.414 0L10 14.586l2.293-2.293a1 1 0 011.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
                      clip-rule="evenodd"
                    />
                  </svg>
                </span>
              </button>

              <div
                class="absolute mt-1 w-full rounded-md bg-white shadow-lg"
                v-if="expandAccounts"
              >
                <ul
                  tabindex="-1"
                  role="listbox"
                  aria-labelledby="listbox-label"
                  aria-activedescendant="listbox-item-3"
                  class="max-h-56 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm"
                >
                  <li
                    v-for="acc in injectedAccounts"
                    v-bind:key="acc.address"
                    @click="selectAccount(acc)"
                    role="option"
                    class="text-gray-900 cursor-default select-none relative py-2 pl-3 pr-9"
                  >
                    <div class="flex items-center">
                      <span class="ml-3 block font-normal truncate">
                        <strong>{{ acc.meta.name }}</strong> | {{ acc.address }}
                      </span>
                    </div>

                    <span
                      class="absolute inset-y-0 right-0 flex items-center pr-4"
                    >
                      <svg
                        class="h-5 w-5"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          fill-rule="evenodd"
                          d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                          clip-rule="evenodd"
                        />
                      </svg>
                    </span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
        <div class="shadow-xl p-10 bg-white rounded">
          <h2 class="text-4xl font-black mb-4">Register</h2>
          <p>Register new member to this contract</p>
          <div class="flex p-5">
            <span
              class="text-sm border border-2 rounded-1 px-4 py-2 bg-gray-300 whitespace-no-wrap"
              >Name</span
            >
            <input
              type="text"
              class="border border-2 rounded-r px-4 py-2 w-full"
              name="Name"
              placeholder="eg: Agus Raharjo"
              v-model="registerName"
            />
          </div>

          <button
            class="inline-flex items-center bg-indigo-600 hover:bg-blue-dark text-white font-bold py-3 px-6 rounded"
            @click="doRegister"
            :disabled="registering"
          >
            <svg
              class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              v-if="registering"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>

            <span>Register</span>
          </button>
        </div>

        <div class="shadow-xl p-10 bg-white rounded">
          <h2 class="text-4xl font-black mb-4">Query Member</h2>
          <div class="flex p-5">
            <span
              class="text-sm border border-2 rounded-l px-4 py-2 bg-gray-300 whitespace-no-wrap"
              >Get Member by Index:</span
            >
            <input
              name="MemberId"
              class="border border-2 rounded-r px-4 py-2 w-full"
              type="text"
              placeholder="Index"
              v-on:keyup="getMemberByIndex"
              v-model="memberIndex"
            />
          </div>
          <div class="flex p-5">
            <span
              class="text-sm border border-2 rounded-l px-4 py-2 bg-gray-300 whitespace-no-wrap"
              >Get Member by Hash:</span
            >
            <input
              name="MemberHash"
              class="border border-2 rounded-r px-4 py-2 w-full"
              type="text"
              placeholder="Hash"
              v-on:keyup="getMemberByHash"
              v-model="memberHash"
            />
          </div>
          
          <div class="grid grid-cols-2 gap-2">
            <div class="flex p-5 left">
            <pre class="left text-left"><code>{{memberData}}</code></pre>
          </div>
          <div v-if="memberHash && memberData.name" class="flex">
            <vue-qrcode :value="memberHash.slice(2)" :options="{ width: 200 }"></vue-qrcode>
          </div>
          </div>

        </div>

        <div
          class="shadow-xl p-10 bg-white rounded row-span-1 col-span-1 box-content"
        >
          <h2 class="text-4xl font-black mb-4">Give Points</h2>

          <div class="flex p-5">
            <span
              class="text-sm border border-2 rounded-l px-4 py-2 bg-gray-300 whitespace-no-wrap"
              >Amount:</span
            >
            <input
              name="MemberAddPoint"
              class="border border-2 rounded-r px-4 py-2 w-full"
              type="text"
              placeholder="amount"
              v-model="addPointCount"
            />
          </div>

          <button
            class="inline-flex items-center bg-indigo-600 hover:bg-blue-dark text-white font-bold py-3 px-6 rounded"
            @click="doAddPoint"
            :disabled="addingPoint"
          >
            <svg
              class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              v-if="addingPoint"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>

            <span>Give to {{ memberData.name }}</span>
          </button>
        </div>

        <div
          class="shadow-xl p-10 bg-white rounded row-span-1 col-span-1 box-content"
        >
          <h2 class="text-4xl font-black mb-4">Use Points</h2>

          <div class="flex p-5">
            <span
              class="text-sm border border-2 rounded-l px-4 py-2 bg-gray-300 whitespace-no-wrap"
              >Amount:</span
            >
            <input
              name="MemberUsePoint"
              class="border border-2 rounded-r px-4 py-2 w-full"
              type="text"
              placeholder="amount"
              v-model="usePointCount"
            />
          </div>

          <button
            class="inline-flex items-center bg-indigo-600 hover:bg-blue-dark text-white font-bold py-3 px-6 rounded"
            @click="doUsePoint"
            :disabled="usingPoint"
          >
            <svg
              class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              v-if="usingPoint"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>

            <span>Use from {{ memberData.name }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import VueQrcode from '@chenfengyuan/vue-qrcode';
export default {
  components: {
    VueQrcode
  },
  data() {
    return {
      connecting: false,
      connected: false,
      registering: false,
      contractAddress: "",
      registerName: "",
      memberCount: 0,
      memberHash: "",
      contract: null,
      alice: null,
      memberData: {},
      errorInfo: "",
      expandAccounts: false,
      injectedAccounts: [],
      selectedAccount: {},
      signer: null,
      memberIndex: 1,
      addPointCount: 0,
      addingPoint: false,
      usePointCount: 0,
      usingPoint: false,
    };
  },
  async mounted() {
    // this.alice = this.$nuchain.keyring.addFromUri("//Alice");
    await this.$nuchain.init();
    this.injectedAccounts = this.$nuchain.getInjectedAccounts();
    if (this.injectedAccounts.length > 0) {
      this.selectedAccount = this.injectedAccounts[0];
      this.selectAccount(this.selectedAccount);
    }

    this.reconnect();
  },
  methods: {
    async reconnect() {
      if (this.contractAddress == "") return;
      this.connecting = true;
      const value = 0;
      const gasLimit = -1;
      await this.$nuchain
        .ConnectNodeAndContract(this.contractAddress)
        .then(async (contract) => {
          // console.log(contract);
          this.contract = contract;
          this.refresh();
          this.connected = true;
        });
      this.connecting = false;
    },
    async getMemberByHash() {
      if (this.contract == null) return;
      this.memberData = (
        await this.contract.query.getMember(
          this.selectedAccount.address,
          { value: 0, gasLimit: -1 },
          this.memberHash
        )
      ).output?.toHuman();
    },
    async getMemberByIndex() {
      console.log(this.memberIndex);
      if (this.contract == null) return;
      this.memberData = {};
      this.memberHash = (
        await this.contract.query.getMemberHash(
          this.selectedAccount.address,
          { value: 0, gasLimit: -1 },
          this.memberIndex
        )
      ).output?.toHuman();
      this.getMemberByHash();
    },
    async doRegister() {
      this.registerName = this.registerName.trim();
      if (this.registerName == "") return;
      console.log(this.selectedAccount.address);
      this.registering = true;
      await this.contract.tx
        .register({ value: 0, gasLimit: -1 }, this.registerName)
        .signAndSend(
          this.selectedAccount.address,
          { signer: this.signer.signer },
          (status) => {
            console.log(status);
            this.errorInfo = "";
            if (status.isInBlock) {
              alert("Member Added!");
              this.refresh();
              this.registering = false;
              console.log(status?.toHuman());
              if (
                status?.events.find((a) => a.event.method == "ExtrinsicFailed")
              ) {
                this.errorInfo =
                  "Operation failed, please check account permission";
              }
            }
          }
        )
        .catch((err) => {
          this.errorInfo = `${err}`;
          this.registering = false;
        });
    },
    async selectAccount(account) {
      this.expandAccounts = false;
      this.selectedAccount = account;
      this.signer = await this.$nuchain.useAccount(
        this.selectedAccount.address
      );
      console.log(account);
    },
    async refresh() {
      this.memberCount =
        (
          await this.contract.query.getMemberCount(this.signer.address, {
            value: 0,
            gasLimit: -1,
          })
        ).output?.toHuman() || 0;
    },
    async doAddPoint() {
      this.addingPoint = true;
      this.contract.tx
        .addPoint(
          { value: 0, gasLimit: -1 },
          this.memberHash,
          this.addPointCount
        )
        .signAndSend(this.selectedAccount.address, this.signer, (status) => {
          this.errorInfo = "";
          if (status.isInBlock) {
            this.getMemberByHash();
            this.addingPoint = false;
            this.addPointCount = 0;
          }
        })
        .catch((err) => {
          this.errorInfo = `${err}`;
          this.addingPoint = false;
        });
    },
    async doUsePoint() {
      this.usingPoint = true;
      this.contract.tx
        .usePoint(
          { value: 0, gasLimit: -1 },
          this.memberHash,
          this.usePointCount
        )
        .signAndSend(this.selectedAccount.address, this.signer, (status) => {
          this.errorInfo = "";
          if (status.isInBlock) {
            this.getMemberByHash();
            this.usePointCount = 0;
            this.usingPoint = false;
          }
        })
        .catch((err) => {
          this.errorInfo = `${err}`;
          this.usingPoint = false;
        });
    },
  },
};
</script>

<style>
/* Sample `apply` at-rules with Tailwind CSS
.container {
@apply min-h-screen flex justify-center items-center text-center mx-auto;
}
*/
.container {
  margin: 0 auto;
  min-height: 100vh;
  /* display: flex; */
  justify-content: center;
  align-items: center;
  text-align: center;
}

.title {
  font-family: "Quicksand", "Source Sans Pro", -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  display: block;
  font-weight: 300;
  font-size: 100px;
  color: #35495e;
  letter-spacing: 1px;
}

.subtitle {
  font-weight: 300;
  font-size: 42px;
  color: #526488;
  word-spacing: 5px;
  padding-bottom: 15px;
}

.links {
  padding-top: 15px;
}
</style>
