obsidian search query="tag:#your-tag" format=json

obsidian search query="/\[\[some_wikilink\]\]/" format=json

obsidian search:context query="/\[\[some_wikilink\]\]/" format=json

obsidian search:context query="section:(#your-tag)" format=json

obsidian outline file="Research Paper" format=md

obsidian property:set file="about" name="agent-status" value="analyzed"

obsidian search query="-[agent-status:analyzed]" format="json"

obsidian search query="[agent-status:analyzed]" format="json"
