import { EfficiencyCodeOptions } from "./EfficiencyCodeOptions";
import { EfficiencyCodeExec } from "./exec";
import { Thread } from "./thread";
import { ThreadOptions } from "./threadOptions";

/**
 * EfficiencyCode is the main class for interacting with the EfficiencyCode agent.
 *
 * Use the `startThread()` method to start a new thread or `resumeThread()` to resume a previously started thread.
 */
export class EfficiencyCode {
  private exec: EfficiencyCodeExec;
  private options: EfficiencyCodeOptions;

  constructor(options: EfficiencyCodeOptions = {}) {
    const { EfficiencyCodePathOverride, env, config } = options;
    this.exec = new EfficiencyCodeExec(EfficiencyCodePathOverride, env, config);
    this.options = options;
  }

  /**
   * Starts a new conversation with an agent.
   * @returns A new thread instance.
   */
  startThread(options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options);
  }

  /**
   * Resumes a conversation with an agent based on the thread id.
   * Threads are persisted in ~/.EfficiencyCode/sessions.
   *
   * @param id The id of the thread to resume.
   * @returns A new thread instance.
   */
  resumeThread(id: string, options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options, id);
  }
}
