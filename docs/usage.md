# Using Shoebox

This guide covers the basic usage of Shoebox for organizing and preserving your videos.

## Accessing the Web Interface

After installing Shoebox, you can access the web interface by navigating to:

```
http://<your-server-address>:3000
```

If you're running Shoebox locally, this would be:

```
http://localhost:3000
```

## Organizing Videos

Shoebox helps you organize your videos by providing a structured way to catalog and tag them.

### Reviewing New Videos

When you first start Shoebox, it will scan your configured media source paths for videos. New videos will appear in the "Unreviewed" section.

1. Navigate to the "Unreviewed" page
2. For each video:
   - Watch the preview
   - Add tags and descriptions
   - Mark as reviewed

### Tagging Videos

Tags help you categorize your videos for easier searching and filtering:

1. Select a video
2. Add relevant tags (e.g., "birthday", "vacation", "family")
3. Save your changes

### Searching and Filtering

You can search for videos based on various criteria:

1. Use the search bar to find videos by name, tag, or description
2. Use filters to narrow down results by date, duration, or other metadata
3. Save your favorite searches for quick access

## Exporting Videos

One of the key features of Shoebox is the ability to export videos for use in external editing tools.

### Basic Export

To export a video:

1. Select the video you want to export
2. Click the "Export" button
3. Choose the export location
4. Wait for the export to complete

The exported video will be copied to the specified location, preserving its original quality.

### Export with Original Path Information

If you've configured Shoebox with original path information (see [Configuration](./configuration.md)), the export will include metadata about the video's original location. This is particularly useful when:

1. You're exporting videos for use in a video editing project
2. You need to maintain references to the original file locations
3. You're migrating videos between systems

## System Information and Management

Shoebox provides system information and management tools to help you maintain your video collection.

### Viewing System Information

To view system information:

1. Navigate to the "System" page
2. Here you can see:
   - Storage usage
   - Number of videos
   - Database status
   - Application version

### Managing Storage

To manage storage:

1. Regularly check the storage usage on the System page
2. Consider archiving older videos if storage is running low
3. Ensure your export and thumbnail directories have sufficient space

## Best Practices

Here are some best practices for using Shoebox effectively:

1. **Regular Reviews**: Set aside time to review new videos regularly
2. **Consistent Tagging**: Develop a consistent tagging system
3. **Backup**: Regularly backup your Shoebox database and configuration
4. **Storage Planning**: Plan your storage needs based on your video collection size
5. **Original Paths**: When possible, configure Shoebox with information about the original location of your videos
