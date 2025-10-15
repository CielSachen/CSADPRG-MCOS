/*
 * Last Names: Panaligan
 * Language: JavaScript
 * Paradigm(s): Procedural, Object-Oriented, Functional
 */

import process from "node:process";
import * as readline from "node:readline/promises";

function printChoices<T>(choices: readonly T[]): void {
  for (const [i, val] of choices.entries()) {
    console.log(`[${i + 1}] ${val}`);
  }
}

async function prompt(msg: string): Promise<string> {
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

  const input = await rl.question(msg);

  rl.close();

  return input;
}

const CURRENCIES = Object.freeze([
  "Philippine Peso (PHP)",
  "United States Dollar (USD)",
  "Japanese Yen (JPY)",
  "British Pound Sterling (GBP)",
  "Euro (EUR)",
  "Chinese Yuan Renminni (CNY)",
]);

const TRANSACTION_TITLES = Object.freeze([
  "Register Account Name",
  "Deposit Amount",
  "Withdraw Amount",
  "Currency Exchange",
  "Record Exchange Rates",
  "Show Interest Amount",
]);

class Account {
  readonly name: string;
  balance: number = 0;
  readonly currency = "PHP";

  constructor(name: string) {
    this.name = name;
  }
}

async function depositBalance(account: Account): Promise<void> {
  console.log(`Current Balance: ${account.balance}`);
  console.log(`Currency: ${account.currency}`);

  console.log();

  try {
    account.balance += Number.parseFloat(await prompt("Deposit Amount: "));

    console.log(`Updated Balance: ${account.balance}`);
  } catch {
    console.log("Deposit amount must be a floating point number!");
  }
}

async function withdrawBalance(account: Account): Promise<void> {
  console.log(`Current Balance: ${account.balance}`);
  console.log(`Currency: ${account.currency}`);

  console.log();

  try {
    account.balance -= Number.parseFloat(await prompt("Withdraw Amount: "));

    console.log(`Updated Balance: ${account.balance}`);
  } catch {
    console.log("Withdraw amount must be a floating point number!");
  }
}

async function exchangeCurrencies(rates: ReadonlyMap<string, number>): Promise<void> {
  console.log("Source Currency Options:");
  printChoices(CURRENCIES);

  console.log();

  let srcIdx: number;

  try {
    srcIdx = Number.parseInt(await prompt("Source Currency: ")) - 1;

    if (srcIdx < 0) {
      // Trigger the error handling (`catch` block).
      throw new TypeError();
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (srcIdx >= CURRENCIES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  let srcAmount: number;

  try {
    srcAmount = Number.parseFloat(await prompt("Source Amount: "));
  } catch {
    console.log("Amount must be a floating point number!");

    return;
  }

  console.log();

  console.log("Exchanged Currency Options:");
  printChoices(CURRENCIES);

  console.log();

  let exchangeIdx: number;

  try {
    exchangeIdx = Number.parseInt(await prompt("Source Currency: ")) - 1;

    if (exchangeIdx < 0) {
      // Trigger the error handling (`catch` block).
      throw new TypeError();
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (exchangeIdx >= CURRENCIES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  const srcPHPAmount = srcAmount * rates.get(CURRENCIES[srcIdx - 1]!)!;
  const exchangeAmount = exchangeIdx === 1 ? srcPHPAmount : srcPHPAmount * rates.get(CURRENCIES[exchangeIdx - 1]!)!;

  console.log(`Exchange Amount: ${exchangeAmount}`);
}

async function setExchangeRates(rates: Map<string, number>): Promise<void> {
  printChoices(CURRENCIES);

  console.log();

  let idx: number;

  try {
    idx = Number.parseInt(await prompt("Source Currency: ")) - 1;

    if (idx < 0) {
      // Trigger the error handling (`catch` block).
      throw new TypeError();
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (idx >= CURRENCIES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  try {
    rates.set(CURRENCIES[idx - 1]!, Number.parseFloat(await prompt("Exchange Rate: ")));
  } catch {
    console.log("Amount must be a floating point number!");
  }
}

const ANNUAL_INTEREST_RATE = 0.05;

async function calculateInterest(account: Readonly<Account>): Promise<void> {
  let currBalance = account.balance;

  console.log(`Current Balance: ${currBalance}`);
  console.log(`Currency: ${account.currency}`);
  console.log(`Interest Rate: ${ANNUAL_INTEREST_RATE * 100}`);

  console.log();

  let dayCnt: number;

  try {
    dayCnt = Number.parseInt(await prompt("Total Number of Days: "));

    if (dayCnt < 0) {
      // Trigger the error handling (`catch` block).
      throw new TypeError();
    }
  } catch {
    console.log("Number must be a positive whole number (integer)!");

    return;
  }

  console.log();

  console.log("Day | Interest | Balance |");

  let dailyInterest = ANNUAL_INTEREST_RATE / 365;

  for (let i = 1; i <= dayCnt; i++) {
    currBalance += currBalance * dailyInterest;

    console.log(
      `${String(i).padEnd(3)} | ${String(dailyInterest.toFixed(2)).padEnd(8)} | ${String(currBalance.toFixed(2)).padEnd(
        7
      )} |`
    );
  }
}

(async function main() {
  const accounts: Account[] = [];
  const exchangeRates = new Map<string, number>();

  for (const currency of CURRENCIES.values().drop(1)) {
    exchangeRates.set(currency, 1.0);
  }

  mainMenu: while (true) {
    console.log("Select Transaction:");
    printChoices(TRANSACTION_TITLES);

    console.log();

    let chosenIdx: number;

    try {
      chosenIdx = Number.parseInt(await prompt("> "), 10);
    } catch {
      chosenIdx = 0;
    }

    console.log();

    if (chosenIdx > 0 && chosenIdx <= TRANSACTION_TITLES.length) {
      console.log(TRANSACTION_TITLES[chosenIdx - 1]);
    }

    switch (chosenIdx) {
      case 1: {
        const account = new Account(await prompt("Account Name: "));

        if (!accounts.some((a) => a.name === account.name)) {
          accounts.push(account);
        } else {
          console.log("An account with this name already exists!");
        }

        break;
      }
      case 2:
      case 3: {
        const accountName = await prompt("Account Name: ");
        const account = accounts.find(async (a) => a.name === accountName);

        if (account) {
          if (chosenIdx === 2) {
            await depositBalance(account);
          } else {
            await withdrawBalance(account);
          }
        } else {
          console.log("No account with this name exists!");
        }

        break;
      }
      case 4:
        currencyExchange: while (true) {
          await exchangeCurrencies(exchangeRates);

          console.log();

          repeatPrompt: while (true) {
            const isRepeating = (await prompt("Convert another currency? (Y/N): ")).toUpperCase();

            if (isRepeating.toUpperCase() === "Y") {
              console.log();

              break repeatPrompt;
            } else if (isRepeating.toUpperCase() === "N") {
              break currencyExchange;
            } else {
              console.log("Only accepting a [Y]es or [N]o answer!");

              console.log();
            }
          }
        }

        break;
      case 5:
        console.log();

        await setExchangeRates(exchangeRates);

        break;
      case 6:
        const accountName = await prompt("Account Name: ");
        const account = accounts.find(async (a) => a.name === accountName);

        if (account) {
          await calculateInterest(account);
        } else {
          console.log("No account with this name exists!");
        }

        break;
      default:
        console.log("No transaction with this ID exists!");

        break;
    }

    console.log();

    exitPrompt: while (true) {
      const isContinuing = (await prompt("Back to the Main Menu (Y/N): ")).toUpperCase();

      if (isContinuing.toUpperCase() === "Y") {
        console.log();

        break exitPrompt;
      } else if (isContinuing.toUpperCase() === "N") {
        break mainMenu;
      } else {
        console.log("Only accepting a [Y]es or [N]o answer!");

        console.log();
      }
    }
  }
})();
