import ky from "ky";
import {Response} from "./types";
import {apiUrl} from "./config";

export class Api {
  static async getRedirectUrl(id: string): Promise<Response> {
    return await (await ky(`${apiUrl}/${id}`, {
      method: 'GET',
    })).json();
  }

  static async sendLongLink(link: string): Promise<Response> {
    return await (await ky(`${apiUrl}/set`,{
      method: 'POST',
      json: {
        link: link,
      }
    })).json();
  }
}
