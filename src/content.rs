pub fn get_command_content(command_name: &str, trigger_name: &str) -> String {
    format!(
r#"import {{ Message }} from 'discord.js';

import Command from './base/Command';

export default class {} implements Command {{
    public readonly TRIGGERS = ['{}'];

    public run(message: Message): void {{
        message.channel.send(this.generateMessage());
    }};
    
    private generateMessage(): string {{
        // @todo return custom message
    }}
}}
    "#, command_name, trigger_name)
}