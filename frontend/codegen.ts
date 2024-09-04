const {default: codegen} = require( '@cosmwasm/ts-codegen');


codegen({
  contracts: [
    {
      name: 'ZuperCampaign',
      dir: './schemas/zuper-campaign'
    },
    {
      name: 'ZuperIndex',
      dir: './schemas/zuper-index'
    }
  ],
  outPath: './src/codegen/',

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: 'bundle.ts',
      scope: 'contracts'
    },
    types: {
      enabled: true
    },
    client: {
      enabled: true
    },
    reactQuery: {
      enabled: false,
      optionalClient: true,
      version: 'v4',
      mutations: true,
      queryKeys: true,
      queryFactory: true,
    },
    recoil: {
      enabled: false
    },
    messageComposer: {
        enabled: true
    },
  }
}).then(() => {
  console.log('✨ all done!');
});