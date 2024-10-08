# Workflow Template Primer

When creating a workflow template, follow these guidelines:

1. Start with a clear task description
2. Break down the process into logical steps
3. For each step:
   - Provide a brief description
   - Include any necessary code snippets
   - Use `<edit>` tags to specify file changes
4. Use consistent formatting throughout
5. Include error handling and edge cases
6. End with a summary of the changes made

Example structure:

```xml
<task_description>
Implement a new feature X
</task_description>

<step>
Step 1: Update configuration file

<edit>
<path>config/settings.yml</path>
<operation>insert_after</operation>
<search>existing_setting: value</search>
<description>Add new feature X configuration</description>
new_feature_x:
  enabled: true
  max_items: 10
</edit>
</step>

<step>
Step 2: Create new module for feature X

<edit>
<path>src/feature_x.rs</path>
<operation>create</operation>
<description>Implement feature X functionality</description>
pub mod feature_x {
    // Implementation details
}
</edit>
</step>

<!-- More steps as needed -->

<summary>
Added configuration for feature X and implemented core functionality.
</summary>
```

Remember to adapt this template to your specific needs and project structure.
