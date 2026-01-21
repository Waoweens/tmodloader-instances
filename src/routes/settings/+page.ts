import { commands } from "$lib/bindings"
import type { PageLoad } from "./$types"

export const load: PageLoad = async () => {
	return {
		config: await commands.getConfig()
	}
}